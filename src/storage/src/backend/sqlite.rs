use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::FromRow;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use uuid::Uuid;

use mate::proto::job::{Job, JobQuery, JobResult};

#[derive(Debug, FromRow)]
pub(crate) struct JobRecord {
    pub id: String,
    pub name: String,
    pub args: String,
    pub status: String,
    pub claimed_at: Option<i64>,
    pub claimed_by: Option<String>,
    pub scheduled_at: i64,
    pub task: String,
    pub started_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub errors: String,
    pub result: Option<String>,
    pub attempts: i64,
    pub max_attempts: i64,
}

impl TryFrom<JobRecord> for Job {
    type Error = anyhow::Error;

    fn try_from(record: JobRecord) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&record.id)?;
        let args = serde_json::from_str(&record.args)?;
        let task = record.task.parse()?;
        let status = record.status.parse()?;
        let scheduled_at = into_system_time(record.scheduled_at)?;
        let completed_at = record.completed_at.map(into_system_time).transpose()?;
        let started_at = record.started_at.map(into_system_time).transpose()?;
        let errors: Vec<String> = serde_json::from_str(&record.errors)?;
        let result: Option<JobResult> = record
            .result
            .map(|r| serde_json::from_str(&r))
            .transpose()?;
        let attempts = record.attempts as u32;
        let max_attempts = record.max_attempts as u32;

        Ok(Job {
            id,
            name: record.name,
            args,
            status,
            scheduled_at,
            task,
            started_at,
            completed_at,
            errors,
            result,
            attempts,
            max_attempts,
        })
    }
}

pub struct SqliteBackend {
    pool: SqlitePool,
}

impl SqliteBackend {
    pub async fn new(url: &str) -> Result<Self> {
        let conn_opts = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(url);
        let pool = SqlitePool::connect_with(conn_opts).await?;

        Self::migrate(&pool).await?;
        Ok(Self { pool })
    }

    async fn migrate(pool: &SqlitePool) -> Result<()> {
        sqlx::migrate!("src/backend/sqlite/migrations")
            .run(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl super::Backend for SqliteBackend {
    async fn create_job(&self, job: Job) -> Result<Job> {
        let id = job.id.to_string();
        let args = serde_json::to_string(&job.args)?;
        let task = job.task.to_string();
        let status = job.status.to_string();
        // FIXME: Store as UTC datetime('now') instead of unix timestamp
        let scheduled_at = into_unix_timestamp(job.scheduled_at)?;
        let max_attempts = job.max_attempts as i64;
        let record = sqlx::query_as!(
            JobRecord,
            r#"
            INSERT INTO jobs (
                id,
                name,
                args,
                status,
                claimed_at,
                claimed_by,
                scheduled_at,
                task,
                started_at,
                completed_at,
                max_attempts
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11
            ) RETURNING *"#,
            id,
            job.name,
            args,
            status,
            Option::<i64>::None,
            Option::<String>::None,
            scheduled_at,
            task,
            Option::<i64>::None,
            Option::<i64>::None,
            max_attempts,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.try_into()?)
    }

    async fn retrieve_jobs(&self, query: JobQuery) -> Result<Vec<Job>> {
        let mut sql = String::from("SELECT * FROM jobs WHERE 1=1");

        if query.status.is_some() {
            sql.push_str(" AND status = ?");
        }

        if query.min_time.is_some() {
            sql.push_str(" AND scheduled_at >= ?");
        }

        if query.max_time.is_some() {
            sql.push_str(" AND scheduled_at <= ?");
        }

        if query.limit.is_some() {
            sql.push_str(" ORDER BY scheduled_at LIMIT ?");
        }

        let mut q = sqlx::query_as::<_, JobRecord>(&sql);

        if let Some(status) = query.status {
            q = q.bind(status.to_string());
        }

        if let Some(min_time) = query.min_time {
            q = q.bind(into_unix_timestamp(min_time)?);
        }

        if let Some(max_time) = query.max_time {
            q = q.bind(into_unix_timestamp(max_time)?);
        }

        if let Some(limit) = query.limit {
            q = q.bind(limit as i64);
        }

        let records = q.fetch_all(&self.pool).await?;
        records.into_iter().map(|r| r.try_into()).collect()
    }

    async fn update_job_completed(&self, id: Uuid, result: JobResult) -> Result<()> {
        let id = id.to_string();
        let result_json = serde_json::to_string(&result)?;

        match &result {
            JobResult::Success(_) => {
                sqlx::query(
                    r#"UPDATE jobs
                        SET
                            status = 'completed',
                            result = ?,
                            completed_at = datetime('now'),
                            attempts = attempts + 1
                        WHERE id = ?"#,
                )
                .bind(result_json)
                .bind(id)
                .execute(&self.pool)
                .await?;
            }
            JobResult::Failure(error) => {
                sqlx::query(
                    r#"UPDATE jobs
                        SET
                            status = 'failed',
                            result = ?,
                            completed_at = datetime('now'),
                            attempts = attempts + 1,
                            errors = json_insert(errors, '$[#]', ?)
                        WHERE id = ?"#,
                )
                .bind(result_json)
                .bind(error)
                .bind(id)
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    async fn claim_jobs(
        &self,
        claimed_by: String,
        count: usize,
        start: SystemTime,
        end: SystemTime,
    ) -> Result<Vec<Job>> {
        let start_ts = into_unix_timestamp(start)?;
        let end_ts = into_unix_timestamp(end)?;
        let count = count as i64;
        let records = sqlx::query_as::<_, JobRecord>(
            r#"
            UPDATE jobs
            SET
                status = 'claimed',
                claimed_at = datetime('now'),
                claimed_by = ?,
                attempts = attempts + 1
            WHERE id IN (
                SELECT id FROM jobs
                WHERE
                    status IN ('scheduled', 'failed')
                    AND attempts < max_attempts
                    AND (
                        scheduled_at BETWEEN ? AND ?
                        OR scheduled_at <= ?
                    )
                ORDER BY scheduled_at
                LIMIT ?
            ) RETURNING *
            "#,
        )
        .bind(claimed_by)
        .bind(start_ts)
        .bind(end_ts)
        .bind(start_ts)
        .bind(count)
        .fetch_all(&self.pool)
        .await?;

        records.into_iter().map(|r| r.try_into()).collect()
    }

    async fn claim_job(&self, job_id: Uuid, _: usize) -> Result<Job> {
        sqlx::query_as::<_, JobRecord>(
            r#"
            UPDATE jobs SET status = 'claimed'
            WHERE
                id = ?
                AND status IN ('scheduled', 'failed')
                AND attempts < max_attempts
            ORDER BY scheduled_at
            LIMIT 1
            RETURNING *
            "#,
        )
        .bind(job_id.to_string())
        .fetch_one(&self.pool)
        .await?
        .try_into()
    }
}

fn into_unix_timestamp(time: SystemTime) -> Result<i64> {
    Ok(time
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("Time went backwards")?
        .as_secs() as i64)
}

fn into_system_time(timestamp: i64) -> Result<SystemTime> {
    SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(timestamp as u64))
        .context("Invalid timestamp")
}

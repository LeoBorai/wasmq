use std::time::SystemTime;

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use uuid::Uuid;

use mate::proto::job::{Job, JobQuery, JobResult};

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
    async fn create_job(&self, _job: Job) -> Result<Job> {
        todo!()
    }

    async fn retrieve_jobs(&self, _query: JobQuery) -> Result<Vec<Job>> {
        Ok(vec![])
    }

    async fn update_job_completed(&self, _id: Uuid, _result: JobResult) -> Result<()> {
        Ok(())
    }

    async fn claim_jobs(
        &self,
        _count: usize,
        _start: SystemTime,
        _end: SystemTime,
    ) -> Result<Vec<Job>> {
        Ok(vec![])
    }
}

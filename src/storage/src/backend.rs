pub mod sqlite;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use mate::proto::job::{Job, JobQuery, JobResult};

#[async_trait]
pub trait Backend: Send + Sync {
    async fn create_job(&self, job: Job) -> Result<Job>;
    async fn retrieve_jobs(&self, query: JobQuery) -> Result<Vec<Job>>;
    async fn update_job_completed(&self, id: Uuid, result: JobResult) -> Result<()>;
    async fn claim_job(&self, job_id: Uuid, claimed_by: String) -> Result<Job>;
}

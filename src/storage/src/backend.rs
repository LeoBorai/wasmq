pub mod sqlite;

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use mate::proto::job::{Job, JobResult};

#[async_trait]
pub trait Backend: Send + Sync {
    async fn create_job(&self, job: Job) -> Result<Job>;
    async fn retrieve_jobs(&self, status: Option<JobStatus>) -> Result<Vec<Job>>;
    async fn update_job_completed(&self, id: Uuid, result: JobResult) -> Result<()>;
}

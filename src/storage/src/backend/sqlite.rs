use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use mate::proto::job::{Job, JobResult};

pub struct SqliteBackend {
}

impl SqliteBackend {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl super::Backend for SqliteBackend {
    async fn create_job(&self, job: Job) -> Result<Job> {
        todo!()
    }

    async fn update_job_completed(&self, id: Uuid, result: JobResult) -> Result<()> {
        Ok(())
    }
}

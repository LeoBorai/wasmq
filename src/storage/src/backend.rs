mod local;

pub use local::LocalBackend;

use anyhow::Result;
use async_trait::async_trait;
use mate::proto::job::Job;

#[async_trait]
pub trait Backend {
    async fn create(&self, job: Job) -> Result<()>;
}

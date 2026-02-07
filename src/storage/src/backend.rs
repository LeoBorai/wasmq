mod local;

pub use local::LocalBackend;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Backend {
    async fn create(&self) -> Result<()>;
}

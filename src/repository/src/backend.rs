mod local;

pub use local::LocalBackend;

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;

use wasmq::proto::task::TaskIdentifier;

#[async_trait]
pub trait Backend {
    async fn create(&self, id: &TaskIdentifier, bytes: Bytes) -> Result<()>;
    async fn find(&self, id: &TaskIdentifier) -> Result<Option<Bytes>>;
    async fn list(&self) -> Result<Vec<TaskIdentifier>>;
}

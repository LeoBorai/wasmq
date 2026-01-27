pub mod backend;

use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;

use mate::proto::task::TaskIdentifier;

use crate::backend::{Backend, LocalBackend};

#[derive(Clone)]
pub struct TaskRepository {
    backend: Arc<dyn Backend + Send + Sync>,
}

impl TaskRepository {
    pub fn new(backend: Arc<dyn Backend + Send + Sync>) -> Self {
        Self { backend }
    }

    pub async fn local() -> Result<Self> {
        let local = LocalBackend::new().await?;
        let backend = Arc::new(local);
        Ok(Self { backend })
    }

    pub async fn store(&self, id: &TaskIdentifier, data: Bytes) -> Result<()> {
        self.backend.create(id, data).await
    }

    pub async fn find(&self, id: &TaskIdentifier) -> Result<Option<Bytes>> {
        self.backend.find(id).await
    }

    pub async fn list(&self) -> Result<Vec<TaskIdentifier>> {
        self.backend.list().await
    }
}

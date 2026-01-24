pub mod backend;

use anyhow::Result;
use bytes::Bytes;

use mate::proto::task::TaskIdentifier;

use crate::backend::{Backend, LocalBackend};

pub struct TaskRepository {
    backend: Box<dyn Backend>,
}

impl TaskRepository {
    pub fn new(backend: Box<dyn Backend>) -> Self {
        Self { backend }
    }

    pub async fn local() -> Result<Self> {
        let local = LocalBackend::new().await?;
        let backend = Box::new(local);
        Ok(Self { backend })
    }

    pub async fn store(&self, id: &TaskIdentifier, data: Bytes) -> Result<()> {
        self.backend.create(id, data).await
    }

    pub async fn find(&self, id: &TaskIdentifier) -> Result<Option<Bytes>> {
        self.backend.find(id).await
    }
}

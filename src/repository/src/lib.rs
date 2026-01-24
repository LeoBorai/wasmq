pub mod backend;
pub mod id;

use anyhow::Result;
use bytes::Bytes;

use crate::{backend::{Backend, LocalBackend}, id::TaskIdentifier};

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
        self.backend.create(&id, data).await
    }
}

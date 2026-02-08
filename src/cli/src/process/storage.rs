use anyhow::Result;

use mate_ipc::transport::Transport;
use mate_storage::Storage;

pub struct StorageProcess {
    storage: Storage,
}

impl StorageProcess {
    pub fn new(transport: Box<dyn Transport>, backend: Arc<dyn Backend + Send + Sync>) -> Self {
        let storage = Storage::new(transport, backend);

        Self { storage }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.storage.run().await
    }
}

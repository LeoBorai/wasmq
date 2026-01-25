use anyhow::Result;

use mate_ipc::transport::Transport;
use mate_storage::Storage;

pub struct StorageProcess {
    storage: Storage,
}

impl StorageProcess {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let storage = Storage::new(transport);

        Self { storage }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.storage.run().await
    }
}

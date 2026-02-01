use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    pub backend: StorageBackend,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StorageBackend {
    Memory,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::Memory,
        }
    }
}

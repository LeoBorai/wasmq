pub mod executor;
pub mod hub;
pub mod scheduler;
pub mod storage;
pub mod transport;

use std::fs::read_to_string;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::executor::ExecutorConfig;
use crate::scheduler::SchedulerConfig;
use crate::storage::StorageConfig;

use self::hub::HubConfig;
use self::transport::TransportConfig;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub transport: TransportConfig,
    pub hub: HubConfig,
    pub storage: StorageConfig,
    pub scheduler: SchedulerConfig,
    pub executors: ExecutorConfig,
}

impl Config {
    /// Creates a [`Config`] instance from a `.toml` file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}

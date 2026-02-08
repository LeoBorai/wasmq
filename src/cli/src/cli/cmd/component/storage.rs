use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tracing::debug;

use mate_config::Config;
use mate_ipc::protocol::ProcessType;
use mate_storage::backend::LocalBackend;

use crate::{process::storage::StorageProcess, transport::make_transport};

#[derive(Debug, Parser)]
pub struct StorageSpawnOpt {
    #[clap(long, short)]
    config: PathBuf,
}

impl StorageSpawnOpt {
    pub async fn exec(&self) -> Result<()> {
        let config = Config::from_file(&self.config)?;
        let transport = make_transport(config.clone(), ProcessType::Storage).await?;
        let backend = LocalBackend::new();
        let mut storage = StorageProcess::new(transport, backend);

        debug!("Starting storage process…");
        storage.run().await?;

        Ok(())
    }
}

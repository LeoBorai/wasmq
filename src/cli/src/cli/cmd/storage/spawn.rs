use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use mate_config::Config;
use mate_ipc::protocol::ProcessType;

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
        let mut storage = StorageProcess::new(transport);

        storage.run().await?;

        Ok(())
    }
}

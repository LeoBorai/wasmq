mod spawn;

use anyhow::Result;
use clap::Parser;

use self::spawn::StorageSpawnOpt;

#[derive(Debug, Parser)]
pub enum StorageCmd {
    Spawn(StorageSpawnOpt),
}

impl StorageCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            StorageCmd::Spawn(cmd) => cmd.exec().await,
        }
    }
}

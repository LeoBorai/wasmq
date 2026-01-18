mod spawn;

use anyhow::Result;
use clap::Parser;

use self::spawn::ExecutorSpawnOpt;

#[derive(Debug, Parser)]
pub enum ExecutorCmd {
    Spawn(ExecutorSpawnOpt),
}

impl ExecutorCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            ExecutorCmd::Spawn(cmd) => cmd.exec().await,
        }
    }
}

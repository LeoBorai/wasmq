mod spawn;

use anyhow::Result;
use clap::Parser;

use self::spawn::SchedulerSpawnOpt;

#[derive(Debug, Parser)]
pub enum SchedulerCmd {
    Spawn(SchedulerSpawnOpt),
}

impl SchedulerCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            SchedulerCmd::Spawn(cmd) => cmd.exec().await,
        }
    }
}

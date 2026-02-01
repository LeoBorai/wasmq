mod executor;
mod scheduler;
mod storage;

use anyhow::Result;
use clap::Parser;

use crate::cli::cmd::component::executor::ExecutorSpawnOpt;
use crate::cli::cmd::component::scheduler::SchedulerSpawnOpt;
use crate::cli::cmd::component::storage::StorageSpawnOpt;

#[derive(Debug, Parser)]
pub enum ComponentCmd {
    Executor(ExecutorSpawnOpt),
    Storage(StorageSpawnOpt),
    Scheduler(SchedulerSpawnOpt),
}

impl ComponentCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            ComponentCmd::Executor(cmd) => cmd.exec().await,
            ComponentCmd::Storage(cmd) => cmd.exec().await,
            ComponentCmd::Scheduler(cmd) => cmd.exec().await,
        }
    }
}

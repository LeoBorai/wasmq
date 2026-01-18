pub mod executor;
pub mod hub;
pub mod scheduler;
pub mod storage;
pub mod task;

use anyhow::Result;
use clap::Parser;

use crate::cli::cmd::executor::ExecutorCmd;
use crate::cli::cmd::scheduler::SchedulerCmd;

use self::hub::HubCmd;
use self::storage::StorageCmd;
use self::task::TaskCmd;

#[derive(Debug, Parser)]
pub enum Cmd {
    /// Executor management
    #[clap(subcommand)]
    Executor(ExecutorCmd),
    /// Hub management
    #[clap(subcommand)]
    Hub(HubCmd),
    /// Scheduler management
    #[clap(subcommand)]
    Scheduler(SchedulerCmd),
    /// Storage management
    #[clap(subcommand)]
    Storage(StorageCmd),
    /// Task management and development
    #[clap(subcommand)]
    Task(TaskCmd),
}

impl Cmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            Cmd::Executor(cmd) => cmd.exec().await,
            Cmd::Hub(cmd) => cmd.exec().await,
            Cmd::Scheduler(cmd) => cmd.exec().await,
            Cmd::Storage(cmd) => cmd.exec().await,
            Cmd::Task(cmd) => cmd.exec().await,
        }
    }
}

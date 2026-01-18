pub mod hub;
pub mod storage;
pub mod task;

use anyhow::Result;
use clap::Parser;

use self::hub::HubCmd;
use self::storage::StorageCmd;
use self::task::TaskCmd;

#[derive(Debug, Parser)]
pub enum Cmd {
    /// Hub management
    #[clap(subcommand)]
    Hub(HubCmd),
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
            Cmd::Hub(cmd) => cmd.exec().await,
            Cmd::Storage(cmd) => cmd.exec().await,
            Cmd::Task(cmd) => cmd.exec().await,
        }
    }
}

pub mod hub;
pub mod task;

use anyhow::Result;
use clap::Parser;

use self::hub::HubCmd;
use self::task::TaskCmd;

#[derive(Debug, Parser)]
pub enum Cmd {
    /// Hub management
    #[clap(subcommand)]
    Hub(HubCmd),
    /// Task management and development
    #[clap(subcommand)]
    Task(TaskCmd),
}

impl Cmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            Cmd::Hub(cmd) => cmd.exec().await,
            Cmd::Task(cmd) => cmd.exec().await,
        }
    }
}

pub mod task;

use anyhow::Result;
use clap::Parser;

use self::task::TaskCmd;

#[derive(Debug, Parser)]
pub enum Cmd {
    /// Task management and development
    #[clap(subcommand)]
    Task(TaskCmd),
}

impl Cmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            Cmd::Task(task_cmd) => task_cmd.exec().await,
        }
    }
}

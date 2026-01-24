mod new;
mod load;
mod run;

use anyhow::Result;
use clap::Parser;

use self::new::TaskNewOpt;
use self::load::TaskLoadOpt;
use self::run::TaskRunOpt;

#[derive(Debug, Parser)]
pub enum TaskCmd {
    /// Create a new Task project
    New(TaskNewOpt),
    /// Loads a Task to the Local Repository so its found by Executors
    Load(TaskLoadOpt),
    /// Run a task passing arguments and retrieves results
    Run(TaskRunOpt),
}

impl TaskCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            TaskCmd::New(cmd) => cmd.exec().await,
            TaskCmd::Load(cmd) => cmd.exec().await,
            TaskCmd::Run(cmd) => cmd.exec().await,
        }
    }
}

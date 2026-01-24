mod list;
mod load;
mod new;
mod run;

use anyhow::Result;
use clap::Parser;

use self::list::TaskListOpt;
use self::load::TaskLoadOpt;
use self::new::TaskNewOpt;
use self::run::TaskRunOpt;

#[derive(Debug, Parser)]
pub enum TaskCmd {
    /// Lists existing Tasks in the Local Repository
    #[clap(alias = "ls")]
    List(TaskListOpt),
    /// Loads a Task to the Local Repository so its found by Executors
    Load(TaskLoadOpt),
    /// Create a new Task project
    New(TaskNewOpt),
    /// Run a task passing arguments and retrieves results
    Run(TaskRunOpt),
}

impl TaskCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            TaskCmd::List(cmd) => cmd.exec().await,
            TaskCmd::Load(cmd) => cmd.exec().await,
            TaskCmd::New(cmd) => cmd.exec().await,
            TaskCmd::Run(cmd) => cmd.exec().await,
        }
    }
}

mod run;

use anyhow::Result;
use clap::Parser;

use self::run::TaskRunOpt;

#[derive(Debug, Parser)]
pub enum TaskCmd {
    /// Run a task passing arguments and retrieves results
    Run(TaskRunOpt),
}

impl TaskCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            TaskCmd::Run(run_cmd) => run_cmd.exec().await,
        }
    }
}

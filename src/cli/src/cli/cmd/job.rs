mod list;
mod new;

use anyhow::Result;
use clap::Parser;

use self::list::JobListOpt;
use self::new::JobNewOpt;

#[derive(Debug, Parser)]
pub enum JobCmd {
    /// Creates a new Job
    New(JobNewOpt),
    /// Lists existing Jobs
    #[clap(alias = "ls")]
    List(JobListOpt),
}

impl JobCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            JobCmd::New(cmd) => cmd.exec().await,
            JobCmd::List(cmd) => cmd.exec().await,
        }
    }
}

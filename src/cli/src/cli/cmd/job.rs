mod list;
mod new;
mod view;

use anyhow::Result;
use clap::Parser;

use crate::cli::cmd::job::view::JobViewOpt;

use self::list::JobListOpt;
use self::new::JobNewOpt;

#[derive(Debug, Parser)]
pub enum JobCmd {
    /// Creates a new Job
    New(JobNewOpt),
    /// Lists Jobs
    #[clap(alias = "ls")]
    List(JobListOpt),
    /// Retrieve a Job details
    View(JobViewOpt),
}

impl JobCmd {
    pub async fn exec(&self) -> Result<()> {
        match &self {
            JobCmd::New(cmd) => cmd.exec().await,
            JobCmd::List(cmd) => cmd.exec().await,
            JobCmd::View(cmd) => cmd.exec().await,
        }
    }
}

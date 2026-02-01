use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use mate_config::Config;
use mate_ipc::protocol::ProcessType;
use tracing::debug;

use crate::process::scheduler::SchedulerProcess;
use crate::transport::make_transport;

#[derive(Debug, Parser)]
pub struct SchedulerSpawnOpt {
    #[clap(long, short)]
    config: PathBuf,
}

impl SchedulerSpawnOpt {
    pub async fn exec(&self) -> Result<()> {
        let config = Config::from_file(&self.config)?;
        let transport = make_transport(config.clone(), ProcessType::Scheduler).await?;
        let mut scheduler = SchedulerProcess::new(transport, 1).await?;

        debug!("Starting scheduler process…");
        scheduler.run().await?;

        Ok(())
    }
}

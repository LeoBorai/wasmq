use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use mate_config::Config;
use mate_ipc::protocol::ProcessType;
use tracing::info;

use crate::process::executor::ExecutorProcess;
use crate::transport::make_transport;

#[derive(Debug, Parser)]
pub struct ExecutorSpawnOpt {
    #[clap(long, short)]
    config: PathBuf,
    #[clap(long, short)]
    id: usize,
}

impl ExecutorSpawnOpt {
    pub async fn exec(&self) -> Result<()> {
        let config = Config::from_file(&self.config)?;
        let transport = make_transport(config.clone(), ProcessType::Executor(self.id)).await?;
        let mut executor = ExecutorProcess::new(transport, self.id).await?;

        info!(id=%self.id, "Starting executor process…");
        executor.run().await?;

        Ok(())
    }
}

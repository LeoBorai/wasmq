use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Parser;
use mate_config::Config;
use mate_ipc::{channel::IpcServer, protocol::ProcessType};

use crate::{process::hub::Hub, server::run_server, transport::make_transport};

#[derive(Debug, Parser)]
pub struct HubStartOpt {
    /// Path to a Mate Config file
    #[clap(long, short)]
    config: PathBuf,
}

impl HubStartOpt {
    pub async fn exec(&self) -> Result<()> {
        let config = Config::from_file(&self.config)?;
        // FIXME: Transport should not know about `ProcessType` it should be only handled by IPC
        let transport = make_transport(config.clone(), ProcessType::Hub).await?;
        let ipc = IpcServer::new(ProcessType::Hub, transport);
        let ipc = Arc::new(ipc);
        let mut hub = Hub::new(self.config.clone());

        hub.spawn_processes().await?;
        run_server(ipc).await?;

        Ok(())
    }
}

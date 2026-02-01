use std::{process::exit, sync::Arc};

use anyhow::Result;
use clap::Parser;

use mate_config::Config;
use mate_repository::TaskRepository;
use tracing::error;

use crate::{process::hub::Hub, server::run_server, utils::shutdown_signal};

#[derive(Debug, Parser)]
pub enum RunCmd {}

impl RunCmd {
    pub async fn run() -> Result<()> {
        // FIXME: Transport should not know about `ProcessType` it should be only handled by IPC
        let mut hub = Hub::new(Config::default()).await?;
        let child_processes = hub.spawn_processes().await?;
        let hub = Arc::new(hub);
        let repo = Arc::new(TaskRepository::local().await?);

        hub.wait_for_components().await?;

        tokio::select! {
            Err(err) = run_server(hub.config(), Arc::clone(&hub), Arc::clone(&repo)) => {
                error!("Server returned an error. {:#?}", err);
            },
            _ = shutdown_signal() => {
                for mut cp in child_processes {
                    if let Err(err) = cp.kill().await {
                        error!("Failed to kill process. {:#?}", err);
                    }
                }

                exit(0);
            },
        }

        Ok(())
    }
}

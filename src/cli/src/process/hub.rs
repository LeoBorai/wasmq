use std::env::current_exe;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::process::{Child, Command};
use tokio::time::sleep;

use mate_config::Config;
use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Message, MessagePayload, ProcessType};

use crate::transport::make_transport;

pub struct Hub {
    config: Config,
    config_path: PathBuf,
    ipc: Arc<IpcServer>,
}

impl Hub {
    pub async fn new(config_path: PathBuf) -> Result<Self> {
        let config = Config::from_file(&config_path)?;
        let transport = make_transport(config.clone(), ProcessType::Hub).await?;
        let ipc = IpcServer::new(ProcessType::Hub, transport);
        let ipc = Arc::new(ipc);

        Ok(Self {
            config,
            config_path,
            ipc,
        })
    }

    #[inline]
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn spawn_processes(&mut self) -> Result<Vec<Child>> {
        let mate_exe = current_exe()?;
        let mut child_processes = Vec::new();
        let storage = Command::new(&mate_exe)
            .arg("storage")
            .arg("spawn")
            .arg("--config")
            .arg(self.config_path.to_str().unwrap())
            .spawn()?;

        child_processes.push(storage);

        let scheduler = Command::new(&mate_exe)
            .arg("scheduler")
            .arg("spawn")
            .arg("--config")
            .arg(self.config_path.to_str().unwrap())
            .spawn()?;

        child_processes.push(scheduler);

        // TODO: Perform Polling via Transport perhaps?
        sleep(Duration::from_secs(1)).await;

        Ok(child_processes)
    }

    pub async fn wait_for_components(&self) -> Result<()> {
        self.ipc
            .request(Message::new(
                ProcessType::Hub,
                ProcessType::Storage,
                MessagePayload::Ping,
            ))
            .await?;

        println!("✓ Storage OK!");

        self.ipc
            .request(Message::new(
                ProcessType::Hub,
                ProcessType::Scheduler,
                MessagePayload::Ping,
            ))
            .await?;

        println!("✓ Scheduler OK!");

        Ok(())
    }

    pub fn ipc(&self) -> Arc<IpcServer> {
        Arc::clone(&self.ipc)
    }
}

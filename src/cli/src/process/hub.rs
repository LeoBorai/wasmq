use std::env::current_exe;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use tokio::process::{Child, Command};
use tokio::time::sleep;

pub struct Hub {
    config: PathBuf,
    child_processes: Vec<Child>,
}

impl Hub {
    pub fn new(config: PathBuf) -> Self {
        Self {
            config,
            child_processes: Vec::new(),
        }
    }

    pub async fn spawn_processes(&mut self) -> Result<()> {
        let mate_exe = current_exe()?;

        let storage = Command::new(&mate_exe)
            .arg("storage")
            .arg("spawn")
            .arg("--config")
            .arg(self.config.to_str().unwrap())
            .spawn()?;

        self.child_processes.push(storage);

        sleep(Duration::from_secs(1)).await; // TODO: Perform Polling via Transport perhaps?

        Ok(())
    }
}

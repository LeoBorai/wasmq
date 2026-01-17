use std::env::current_exe;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::process::{Child, Command};
use tokio::time::sleep;

use mate_ipc::channel::IpcServer;

pub struct Hub {
    config: PathBuf,
    child_processes: Vec<Child>,
    ipc: Arc<IpcServer>,
}

impl Hub {
    pub fn new(config: PathBuf, ipc: Arc<IpcServer>) -> Self {
        Self {
            config,
            child_processes: Vec::new(),
            ipc,
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

        // let scheduler = Command::new(&mate_exe)
        //     .arg("--process=scheduler")
        //     .spawn()?;
        // self.child_processes.push(scheduler);

        // for i in 0..4 {
        //     let executor = Command::new(&mate_exe)
        //         .arg(format!("--process=executor:{}", i))
        //         .spawn()?;
        //     self.child_processes.push(executor);
        // }

        sleep(Duration::from_secs(1)).await; // TODO: Perform Polling via Transport perhaps?

        Ok(())
    }
}

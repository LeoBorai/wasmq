use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tokio::fs::read;

use mate::{client::Client, proto::task::TaskIdentifier};

#[derive(Debug, Parser)]
pub struct TaskLoadOpt {
    pub path: PathBuf,
    /// Name for the Task to be loaded
    #[clap(long)]
    pub id: TaskIdentifier,
}

impl TaskLoadOpt {
    pub async fn exec(&self) -> Result<()> {
        let wasm = read(&self.path).await?;
        let client = Client::new("http://localhost:6283");

        match client.api.v0.tasks.create(&self.id, wasm).await {
            Ok(()) => {
                println!(
                    "Task \"{}\" has been loaded into local the repository.",
                    self.id
                );
            }
            Err(e) => {
                eprintln!("Failed to load task: {}", e);
            }
        }

        Ok(())
    }
}

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use mate_repository::{TaskRepository, id::TaskIdentifier};
use tokio::fs::read;

#[derive(Debug, Parser)]
pub struct TaskLoadOpt {
    pub path: PathBuf,
    /// Name for the Task to be loaded
    #[clap(long)]
    pub id: TaskIdentifier,
}

impl TaskLoadOpt {
    pub async fn exec(&self) -> Result<()> {
        let repo = TaskRepository::local().await?;
        let wasm = read(&self.path).await?;

        repo.store(&self.id, wasm.into()).await?;
        println!(
            "Task \"{}\" has been loaded into local the repository.",
            self.id
        );

        Ok(())
    }
}

use std::env::current_dir;

use anyhow::{Context, Result, bail};
use clap::Parser;
use include_dir::{Dir, include_dir};
use tokio::fs::create_dir_all;

use mate_repository::TaskIdentifier;

static ASSETS_TASK_RUST: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/task/rust");

#[derive(Debug, Parser)]
pub struct TaskNewOpt {
    pub name: TaskIdentifier,
}

impl TaskNewOpt {
    pub async fn exec(&self) -> Result<()> {
        let current_dir = current_dir().context("Failed to get current directory")?;
        let task_dir = current_dir.join(&self.name.name);

        if task_dir.exists() {
            bail!("Task directory '{}' already exists.", task_dir.display());
        }

        create_dir_all(&task_dir)
            .await
            .with_context(|| format!("Failed to create task directory '{}'", task_dir.display()))?;

        for _ in ASSETS_TASK_RUST.entries() {
            // Recursively replicate directory structure and files
            // Should replace certain text with project-specific values
            // MATE_TASK_NAME, MATE_TASK_AUTHOR, MATE_TASK_VERSION
        }

        Ok(())
    }
}

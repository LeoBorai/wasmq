use std::fs::read;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;

use mate_executor::Executor;

#[derive(Debug, Parser)]
pub struct TaskNewOpt {}

impl TaskNewOpt {
    pub async fn exec(&self) -> Result<()> {
        Ok(())
    }
}

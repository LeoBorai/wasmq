use std::fs::create_dir_all;
use std::io::Write;
use std::path::PathBuf;
use std::{env::current_dir, path::Path};

use anyhow::{Context, Result, bail};
use clap::Parser;
use include_dir::{Dir, File, include_dir};

use mate::proto::task::TaskIdentifier;

static ASSETS_TASK_RUST: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/task/rust");

#[derive(Debug, Parser)]
pub struct TaskNewOpt {
    pub name: TaskIdentifier,
}

impl TaskNewOpt {
    pub async fn exec(&self) -> Result<()> {
        let current_dir = current_dir().context("Failed to get current directory")?;
        let target_dir = current_dir.join(&self.name.name);

        if target_dir.exists() {
            bail!("Task directory '{}' already exists.", target_dir.display());
        }

        create_dir_all(&target_dir).with_context(|| {
            format!("Failed to create task directory '{}'", target_dir.display())
        })?;

        Self::copy_dir(&ASSETS_TASK_RUST, &target_dir)?;

        Ok(())
    }

    fn copy_dir<'a>(dir: &Dir<'a>, target: &PathBuf) -> Result<()> {
        for entry in dir.entries() {
            if let Some(dir) = entry.as_dir() {
                let dir_path = target.join(dir.path());

                if !dir_path.exists() {
                    create_dir_all(&dir_path)?;
                }

                Self::copy_dir(dir, target)?;
            }

            if let Some(file) = entry.as_file() {
                Self::copy_file(file, target)?;
            }
        }

        Ok(())
    }

    fn copy_file<'a>(entry: &File<'a>, target: &Path) -> Result<()> {
        let target_path = target.join(entry.path());
        let mut file = std::fs::File::create_new(target_path)?;
        file.write_all(entry.contents())?;
        Ok(())
    }
}

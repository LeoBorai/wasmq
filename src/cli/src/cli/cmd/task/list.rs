use anyhow::Result;
use clap::Parser;

use mate_repository::TaskRepository;

#[derive(Debug, Parser)]
pub struct TaskListOpt {}

impl TaskListOpt {
    pub async fn exec(&self) -> Result<()> {
        let repo = TaskRepository::local().await?;
        let tasks = repo.list().await?;

        for task in tasks {
            println!("{}", task);
        }

        Ok(())
    }
}

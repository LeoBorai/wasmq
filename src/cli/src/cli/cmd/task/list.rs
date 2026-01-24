use anyhow::Result;
use clap::Parser;
use tabled::Tabled;

use mate_repository::TaskRepository;

use crate::cli::utils::display::print_table;

#[derive(Tabled)]
struct TaskListItem {
    namespace: String,
    name: String,
    version: String,
}

#[derive(Debug, Parser)]
pub struct TaskListOpt {}

impl TaskListOpt {
    pub async fn exec(&self) -> Result<()> {
        let repo = TaskRepository::local().await?;
        let tasks = repo.list().await?;
        let tasks: Vec<TaskListItem> = tasks
            .into_iter()
            .map(|task| TaskListItem {
                namespace: task.namespace,
                name: task.name,
                version: task.version.to_string(),
            })
            .collect();

        print_table(tasks);
        Ok(())
    }
}

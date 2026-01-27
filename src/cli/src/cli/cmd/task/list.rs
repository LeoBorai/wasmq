use anyhow::Result;
use clap::Parser;
use tabled::Tabled;

use mate::client::Client;

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
        let client = Client::new("http://localhost:6283");

        match client.api.v0.tasks.retrieve().await {
            Ok(tasks) => {
                let data: Vec<TaskListItem> = tasks
                    .into_iter()
                    .map(|task| TaskListItem {
                        namespace: task.namespace,
                        name: task.name,
                        version: task.version.to_string(),
                    })
                    .collect();

                print_table(data);
            }
            Err(e) => {
                println!("Failed to list tasks: {}", e);
            }
        }

        Ok(())
    }
}

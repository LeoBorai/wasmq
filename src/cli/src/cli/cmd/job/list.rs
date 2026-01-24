use anyhow::Result;
use clap::Parser;
use tabled::Tabled;

use mate::Client;
use uuid::Uuid;

use crate::cli::utils::display::print_table;

#[derive(Tabled)]
struct JobListItem {
    id: Uuid,
    name: String,
    status: String,
    task: String,
    result: String,
    retries: String,
}

// pub id: Uuid,
// pub name: String,
// pub payload: Value,
// pub status: JobStatus,
// pub scheduled_at: SystemTime,
// pub task: TaskIdentifier,
// pub started_at: Option<SystemTime>,
// pub completed_at: Option<SystemTime>,
// pub errors: Vec<String>,
// pub result: Option<JobResult>,
// pub retry_count: u32,
// pub max_retries: u32,

#[derive(Debug, Parser)]
pub struct JobListOpt {}

impl JobListOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:8080".to_string());

        match client.retrieve_jobs().await {
            Ok(jobs) => {
                let data = jobs
                    .into_iter()
                    .map(|job| JobListItem {
                        id: job.id,
                        name: job.name,
                        status: format!("{}", job.status),
                        task: format!("{}", job.task),
                        result: match &job.result {
                            Some(res) => format!("{}", res),
                            None => "N/A".to_string(),
                        },
                        retries: format!("{}/{}", job.retry_count, job.max_retries),
                    })
                    .collect::<Vec<JobListItem>>();

                print_table(data);
            }
            Err(e) => {
                println!("Failed to list jobs: {}", e);
            }
        }

        Ok(())
    }
}

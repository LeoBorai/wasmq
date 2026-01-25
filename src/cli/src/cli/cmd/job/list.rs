use std::time::SystemTime;

use anyhow::Result;
use clap::Parser;
use tabled::Tabled;

use mate::{Client, proto::job::JobStatus};
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
    tte: String,
}

#[derive(Debug, Parser)]
pub struct JobListOpt {}

impl JobListOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:8080".to_string());

        match client.retrieve_jobs().await {
            Ok(jobs) => {
                let data = jobs
                    .into_iter()
                    .map(|job| {
                        let tte_duration = job
                            .scheduled_at
                            .duration_since(SystemTime::now())
                            .unwrap_or_default();
                        let tte_label = if job.status == JobStatus::Completed {
                            "--".to_string()
                        } else if tte_duration.as_secs() == 0 {
                            "Due".to_string()
                        } else {
                            humantime::format_duration(tte_duration)
                                .to_string()
                                .split(' ')
                                .next()
                                .unwrap_or("N/A")
                                .to_string()
                        };

                        JobListItem {
                            id: job.id,
                            name: job.name,
                            status: format!("{}", job.status),
                            task: format!("{}", job.task),
                            result: match &job.result {
                                Some(res) => format!("{}", res),
                                None => "N/A".to_string(),
                            },
                            retries: format!("{}/{}", job.retry_count, job.max_retries),
                            tte: tte_label,
                        }
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

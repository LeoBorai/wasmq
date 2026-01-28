use std::time::SystemTime;

use anyhow::Result;
use clap::Parser;
use tabled::Tabled;
use uuid::Uuid;

use mate::client::Client;
use mate::proto::job::JobStatus;

use crate::cli::utils::display::print_table;

#[derive(Tabled)]
struct JobListItem {
    id: Uuid,
    name: String,
    status: String,
    task: String,
    result: String,
    attempts: String,
    tte: String,
}

#[derive(Debug, Parser)]
pub struct JobListOpt {
    /// List all Jobs
    #[clap(long, short)]
    all: bool,
    /// Filter by Job Status
    #[clap(long)]
    status: Option<JobStatus>,
}

impl JobListOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:6283");

        match client.api.v0.jobs.retrieve().await {
            Ok(jobs) => {
                let data = jobs
                    .into_iter()
                    .filter(|job| {
                        if self.all {
                            return true;
                        }

                        if let Some(filter_status) = &self.status {
                            return &job.status == filter_status;
                        }

                        job.status != JobStatus::Completed
                    })
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
                            attempts: format!("{}/{}", job.attempts, job.max_attempts),
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

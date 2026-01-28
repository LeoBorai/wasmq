use anyhow::Result;
use clap::Parser;
use serde_json::Value;

use mate::client::Client;
use mate::client::api::v0::jobs::CreateJobRequest;
use mate::proto::task::TaskIdentifier;

use crate::cli::utils::io::parse_json;

#[derive(Debug, Parser)]
pub struct JobNewOpt {
    /// Name of the job
    #[clap(long, short)]
    pub name: String,
    /// Arguments for the job in JSON format
    #[clap(long, short, value_parser = parse_json)]
    pub args: Value,
    /// Task to execute this job with
    #[clap(long, short)]
    pub task: TaskIdentifier,
    /// Maximum number of attempts for the job
    #[clap(long, short, default_value_t = 3)]
    pub max_attempts: u32,
}

impl JobNewOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:6283");

        match client
            .api
            .v0
            .jobs
            .create(CreateJobRequest {
                name: self.name.clone(),
                task: self.task.clone(),
                args: self.args.clone(),
                max_attempts: Some(self.max_attempts),
            })
            .await
        {
            Ok(job) => {
                println!("Job created successfully. ID: {:?}", job.id);
            }
            Err(e) => {
                println!("Failed to create job: {}", e);
            }
        }

        Ok(())
    }
}

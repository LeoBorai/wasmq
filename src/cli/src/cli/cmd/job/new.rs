use anyhow::Result;
use clap::Parser;
use serde_json::Value;

use mate::client::Client;
use mate::proto::task::TaskIdentifier;

use crate::cli::utils::io::parse_json;

#[derive(Debug, Parser)]
pub struct JobNewOpt {
    /// Name of the job
    #[clap(long, short)]
    pub name: String,
    /// Payload for the job in JSON format
    #[clap(long, short, value_parser = parse_json)]
    pub payload: Value,
    /// Task to execute this job with
    #[clap(long, short)]
    pub task: TaskIdentifier,
}

impl JobNewOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:6283");

        match client
            .api
            .v0
            .jobs
            .create(self.name.clone(), self.task.clone(), self.payload.clone())
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

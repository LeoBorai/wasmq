use anyhow::Result;
use clap::Parser;
use serde_json::Value;

use mate::{Client, proto::task::TaskIdentifier};

#[derive(Debug, Parser)]
pub struct JobNewOpt {
    /// Name of the job
    #[clap(long, short)]
    pub name: String,
    /// Payload for the job in JSON format
    #[clap(long, short)]
    pub payload: Value,
    /// Task to execute this job with
    #[clap(long, short)]
    pub task: TaskIdentifier,
}

impl JobNewOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:8080".to_string());

        match client
            .create_job(self.name.clone(), self.task.clone(), self.payload.clone())
            .await
        {
            Ok(job) => {
                println!("Job created successfully: {:?}", job);
            }
            Err(e) => {
                println!("Failed to create job: {}", e);
            }
        }

        Ok(())
    }
}

use anyhow::Result;
use clap::Parser;
use ulid::Ulid;

use wasmq::client::Client;

#[derive(Debug, Parser)]
pub struct JobViewOpt {
    /// Job ID
    id: Ulid,
}

impl JobViewOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:6283");

        match client.api.v0.jobs.find_by_id(self.id).await {
            Ok(Some(job)) => {
                let job_json = serde_json::to_string_pretty(&job)?;
                println!("{job_json}");
            }
            Ok(None) => {
                println!("Job with ID {} not found.", self.id);
            }
            Err(e) => {
                println!("Failed to list jobs: {}", e);
            }
        }

        Ok(())
    }
}

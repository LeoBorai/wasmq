use anyhow::Result;
use clap::Parser;
use uuid::Uuid;

use mate::Client;

#[derive(Debug, Parser)]
pub struct JobViewOpt {
    /// Job ID
    id: Uuid,
}

impl JobViewOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:8080".to_string());

        match client.find_job_by_id(self.id).await {
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

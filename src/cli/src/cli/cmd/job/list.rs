use anyhow::Result;
use clap::Parser;

use mate::Client;

#[derive(Debug, Parser)]
pub struct JobListOpt {}

impl JobListOpt {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::new("http://localhost:8080".to_string());

        match client.retrieve_jobs().await {
            Ok(jobs) => {
                for job in jobs {
                    println!("{:?}", job);
                }
            }
            Err(e) => {
                println!("Failed to list jobs: {}", e);
            }
        }

        Ok(())
    }
}

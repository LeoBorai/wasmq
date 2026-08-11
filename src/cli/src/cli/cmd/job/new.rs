use anyhow::{Result, bail};
use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use serde_json::Value;

use wasmq::client::Client;
use wasmq::client::api::v0::jobs::CreateJobRequest;
use wasmq::proto::task::TaskIdentifier;

use crate::cli::utils::io::parse_json;

#[derive(Debug, Parser)]
pub struct JobNewOpt {
    #[clap(long, short, help = "Job's name")]
    pub name: String,
    #[clap(long, short, value_parser = parse_json, help = "Arguments for the Job in JSON format")]
    pub args: Value,
    #[clap(long, short, help = "Task to execute this Job with")]
    pub task: TaskIdentifier,
    #[clap(
        long,
        short,
        default_value_t = 3,
        help = "Maximum number of attempts for the job"
    )]
    pub max_attempts: u32,
    #[clap(long, help = "Scheduled time for the job in RFC3339 format")]
    pub scheduled_at: String,
}

impl JobNewOpt {
    pub async fn exec(&self) -> Result<()> {
        let scheduled_at = self.parse_scheduled_at()?;
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
                scheduled_at,
            })
            .await
        {
            Ok(job) => {
                println!("{}", job.id.to_string());
            }
            Err(e) => {
                println!("Failed to create job: {}", e);
            }
        }

        Ok(())
    }

    /// Attempts to parse the `scheduled_at` field into a `chrono::DateTime<Utc>`.
    ///
    /// Theres 2 possible inputs:
    ///
    /// - A valid RFC3339 datetime string.
    /// - A relative duration string (e.g., "5m", "2h").
    fn parse_scheduled_at(&self) -> Result<DateTime<Utc>> {
        if self.scheduled_at.is_empty() {
            bail!("The `scheduled_at` value must be provided and cannot be empty.");
        }

        // Try to parse as RFC3339 datetime
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&self.scheduled_at) {
            return Ok(dt.with_timezone(&chrono::Utc));
        }

        if let Ok(duration) = humantime::parse_duration(&self.scheduled_at) {
            let scheduled_time = Utc::now() + Duration::from_std(duration)?;
            return Ok(scheduled_time);
        }

        bail!(
            "Invalid `scheduled_at` value. It must be either a valid RFC3339 datetime string or a relative duration string (e.g., \"5m\", \"2h\")."
        );
    }
}

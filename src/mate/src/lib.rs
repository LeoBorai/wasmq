//! Mate Client
//!
//! ```ignore
//! use mate::JobClient;
//!
//! let client = JobClient::new("http://localhost:3000".to_string());
//! let payload = serde_json::json!({
//!     "data": "example"
//! });
//!
//! let message = client
//!     .create_job(
//!         "my-job".to_string(),
//!         "task::identifier".to_string(),
//!         payload,
//!     )
//!     .await?;
//!
//! println!("Created job: {:?}", message);
//! Ok(())
//! ```

pub mod proto;

use anyhow::{Result, bail};
use reqwest::Client as HttpClient;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::proto::job::Job;
use crate::proto::task::TaskIdentifier;

#[derive(Debug, Serialize)]
struct CreateJobRequest {
    name: String,
    task: String,
    payload: Value,
}

pub struct Client {
    client: HttpClient,
    base_url: String,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Self {
            client: HttpClient::new(),
            base_url,
        }
    }

    pub async fn create_job(
        &self,
        name: String,
        task: TaskIdentifier,
        payload: Value,
    ) -> Result<Job> {
        let task = task.to_string();
        let request = CreateJobRequest {
            name,
            task,
            payload,
        };

        let response = self
            .client
            .post(format!("{}/api/v0/jobs", self.base_url)) // Adjust the path as needed
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let message = response.json::<Job>().await?;
            return Ok(message);
        }

        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }

    pub async fn retrieve_jobs(&self) -> Result<Vec<Job>> {
        let request = self.client.get(format!("{}/api/v0/jobs", self.base_url));
        let response = request.send().await?;

        if response.status().is_success() {
            let jobs = response.json::<Vec<Job>>().await?;
            return Ok(jobs);
        }
        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }

    pub async fn find_job_by_id(&self, id: Uuid) -> Result<Option<Job>> {
        let request = self
            .client
            .get(format!("{}/api/v0/jobs?id={}", self.base_url, id));
        let response = request.send().await?;

        if response.status().is_success() {
            let jobs = response.json::<Vec<Job>>().await?;

            if let Some(job) = jobs.into_iter().find(|job| job.id == id) {
                return Ok(Some(job));
            }

            return Ok(None);
        }

        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }
}

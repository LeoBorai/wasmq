use std::sync::Arc;

use anyhow::{Result, bail};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    client::HttpClient,
    proto::{job::Job, task::TaskIdentifier},
};

#[derive(Debug, Serialize)]
struct CreateJobRequest {
    name: String,
    task: String,
    payload: Value,
}

pub struct RetrieveJobsQuery {
    pub id: Option<Uuid>,
}

#[derive(Clone)]
pub struct ApiV0Jobs {
    http_client: Arc<HttpClient>,
}

impl ApiV0Jobs {
    pub(super) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    pub async fn create(&self, name: String, task: TaskIdentifier, payload: Value) -> Result<Job> {
        let task = task.to_string();
        let request = CreateJobRequest {
            name,
            task,
            payload,
        };
        let response = self
            .http_client
            .client
            .post(format!("{}/api/v0/jobs", self.http_client.base_url)) // Adjust the path as needed
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

    pub async fn retrieve(&self) -> Result<Vec<Job>> {
        let request = self
            .http_client
            .client
            .get(format!("{}/api/v0/jobs", self.http_client.base_url));
        let response = request.send().await?;

        if response.status().is_success() {
            let jobs = response.json::<Vec<Job>>().await?;
            return Ok(jobs);
        }
        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Job>> {
        let request = self.http_client.client.get(format!(
            "{}/api/v0/jobs?id={}",
            self.http_client.base_url, id
        ));
        let response = request.send().await?;

        if response.status().is_success() {
            let jobs = response.json::<Vec<Job>>().await?;
            return Ok(jobs.first().cloned());
        }
        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }
}

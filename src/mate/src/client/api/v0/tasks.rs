use std::sync::Arc;

use anyhow::{Result, bail};
use reqwest::multipart::{Form, Part};
use ulid::Ulid;

use crate::client::HttpClient;
use crate::proto::task::TaskIdentifier;

pub struct RetrieveJobsQuery {
    pub id: Option<Ulid>,
}

#[derive(Clone)]
pub struct ApiV0Tasks {
    http_client: Arc<HttpClient>,
}

impl ApiV0Tasks {
    pub(super) fn new(http_client: Arc<HttpClient>) -> Self {
        Self { http_client }
    }

    pub async fn create(&self, id: &TaskIdentifier, bytes: Vec<u8>) -> Result<()> {
        let form = Form::new().part(
            "task",
            Part::bytes(bytes)
                .file_name("task.wasm")
                .mime_str("application/octet-stream")?,
        );
        let response = self
            .http_client
            .client
            .post(format!(
                "{}/api/v0/tasks/{}/{}/{}",
                self.http_client.base_url, id.namespace, id.name, id.version
            ))
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            return Ok(());
        }

        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }

    pub async fn retrieve(&self) -> Result<Vec<TaskIdentifier>> {
        let request = self
            .http_client
            .client
            .get(format!("{}/api/v0/tasks", self.http_client.base_url));
        let response = request.send().await?;

        if response.status().is_success() {
            let tasks = response.json::<Vec<TaskIdentifier>>().await?;
            return Ok(tasks);
        }
        let status = response.status();
        let error_text = response.text().await?;

        bail!("Request failed with status {}: {}", status, error_text)
    }
}

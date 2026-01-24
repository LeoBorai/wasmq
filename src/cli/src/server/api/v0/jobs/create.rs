use std::str::FromStr;
use std::time::SystemTime;

use axum::http::StatusCode;
use axum::{Extension, Json};
use mate::proto::task::TaskIdentifier;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use mate::proto::job::{Job, JobStatus};
use mate_ipc::protocol::{Message, MessagePayload, ProcessType};

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    name: String,
    task: String,
    payload: Value,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Json(job_data): Json<CreateJobRequest>,
) -> Result<Json<Job>, ApiError> {
    let task = TaskIdentifier::from_str(&job_data.task).map_err(|_| ApiError {
        message: String::from("Invalid task identifier format"),
        status: StatusCode::BAD_REQUEST,
    })?;

    if job_data.name.is_empty() || job_data.name.contains(' ') {
        return Err(ApiError {
            message: String::from("Job name cannot contain spaces and cannot be empty"),
            status: StatusCode::BAD_REQUEST,
        });
    }

    if job_data.task.is_empty() || job_data.task.contains(' ') {
        return Err(ApiError {
            message: String::from("Job task cannot contain spaces and cannot be empty"),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let job = Job {
        id: Uuid::new_v4(),
        name: job_data.name,
        payload: job_data.payload,
        status: JobStatus::Scheduled,
        scheduled_at: SystemTime::now(),
        started_at: None,
        completed_at: None,
        result: None,
        retry_count: 0,
        max_retries: 3,
        task,
        errors: Vec::new(),
    };
    let msg = Message::new(
        ProcessType::Hub,
        ProcessType::Storage,
        MessagePayload::StoreJob(job.clone()),
    );
    let message = services
        .hub
        .ipc()
        .request(msg)
        .await
        .map_err(|e| e.to_string())
        .map_err(|err| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err,
        })?;

    match message.payload {
        MessagePayload::JobStored(Ok(job)) => Ok(Json(job)),
        MessagePayload::JobStored(Err(message)) => Err(ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }),
        _ => Err(ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: String::from("Unexpected response from storage service"),
        }),
    }
}

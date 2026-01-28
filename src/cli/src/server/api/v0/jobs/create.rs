use std::time::{Duration, SystemTime};

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
    task: TaskIdentifier,
    args: Value,
    max_attempts: Option<u32>,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Json(job_data): Json<CreateJobRequest>,
) -> Result<Json<Job>, ApiError> {
    if job_data.name.is_empty() || job_data.name.contains(' ') {
        return Err(ApiError {
            message: String::from("Job name cannot contain spaces and cannot be empty"),
            status: StatusCode::BAD_REQUEST,
        });
    }

    let job = Job {
        id: Uuid::new_v4(),
        name: job_data.name,
        args: job_data.args,
        status: JobStatus::Scheduled,
        scheduled_at: SystemTime::now() + Duration::from_secs(5),
        started_at: None,
        completed_at: None,
        result: None,
        attempts: 0,
        max_attempts: job_data.max_attempts.unwrap_or(3),
        task: job_data.task,
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

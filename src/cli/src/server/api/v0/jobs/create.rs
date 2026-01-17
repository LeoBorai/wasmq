use std::time::SystemTime;

use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use mate_ipc::protocol::{Job, JobStatus, Message, MessagePayload, ProcessType};

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    name: String,
    payload: serde_json::Value,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Json(job_data): Json<CreateJobRequest>,
) -> Result<Json<Message>, ApiError> {
    let job = Job {
        id: Uuid::new_v4(),
        name: job_data.name,
        payload: job_data.payload,
        status: JobStatus::Pending,
        scheduled_at: SystemTime::now(),
        started_at: None,
        completed_at: None,
        result: None,
        retry_count: 0,
        max_retries: 3,
    };
    let msg = Message {
        id: Uuid::new_v4(),
        from: ProcessType::Hub,
        to: ProcessType::Storage,
        payload: MessagePayload::StoreJob(job.clone()),
        reply_to: None,
    };
    let message = services
        .ipc_server
        .request(msg)
        .await
        .map_err(|e| e.to_string())
        .map_err(|err| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: err,
        })?;

    Ok(Json(message))
}

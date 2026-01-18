use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use mate_ipc::protocol::{Job, JobQuery, JobStatus, Message, MessagePayload, ProcessType};

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Deserialize)]
pub struct RetrieveJobsQuery {
    status: Option<JobStatus>,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Query(query): Query<RetrieveJobsQuery>,
) -> Result<Json<Vec<Job>>, ApiError> {
    let msg = Message {
        id: Uuid::new_v4(),
        from: ProcessType::Hub,
        to: ProcessType::Storage,
        payload: MessagePayload::QueryJobs(JobQuery {
            status: query.status,
        }),
        reply_to: None,
    };

    let response = services
        .hub
        .ipc()
        .request(msg)
        .await
        .map_err(|e| ApiError {
            message: e.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    match response.payload {
        MessagePayload::JobsResult(jobs) => Ok(Json(jobs)),
        _ => Err(ApiError {
            message: "Unexpected response".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

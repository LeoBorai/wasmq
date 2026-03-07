use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;
use uuid::Uuid;

use mate::proto::job::{Job, JobQuery, JobStatus};
use mate_ipc::protocol::{HubMessage, Message, MessagePayload, ProcessType, StorageMessage};

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Deserialize)]
pub struct RetrieveJobsQuery {
    id: Option<Uuid>,
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
        payload: HubMessage::QueryJobs(JobQuery {
            status: query.status,
            time_range: None,
        })
        .into(),
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
        MessagePayload::Storage(StorageMessage::JobsResult(jobs)) => {
            if let Some(id) = query.id {
                let jobs: Vec<Job> = jobs.into_iter().filter(|job| job.id == id).collect();
                return Ok(Json(jobs));
            }

            Ok(Json(jobs))
        }
        _ => Err(ApiError {
            message: "Unexpected response".into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

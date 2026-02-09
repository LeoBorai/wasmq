use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use mate::proto::job::{Job, JobQuery, JobStatus};
use mate_ipc::protocol::{Message, MessagePayload, ProcessType};

use crate::process::hub::IPC_SENDER_HUB;
use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Serialize)]
pub struct HealthResponse {
    pub storage: bool,
    pub schoduler: bool,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
) -> Result<Json<HealthResponse>, ApiError> {
    let storage = services.hub.ipc()
        .request(Message::new(
            IPC_SENDER_HUB,
            ProcessType::Storage,
            MessagePayload::Ping,
        ))
        .await;

    let storage = services.hub.ipc()
        .request(Message::new(
            IPC_SENDER_HUB,
            ProcessType::Scheduler,
            MessagePayload::Ping,
        ))
        .await;

    Ok(Json(HealthResponse {
        storage: storage.is_ok(),
        schoduler: storage.is_ok(),
    }))
}

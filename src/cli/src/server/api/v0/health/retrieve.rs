use axum::{Extension, Json};
use serde::Serialize;

use mate_ipc::protocol::{Message, MessagePayload, ProcessType};

use crate::process::hub::IPC_SENDER_HUB;
use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Serialize)]
pub struct HealthResponse {
    pub storage: bool,
    pub scheduler: bool,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
) -> Result<Json<HealthResponse>, ApiError> {
    let ipc = services.hub.ipc();
    let storage = ipc
        .request(Message::new(
            IPC_SENDER_HUB,
            ProcessType::Storage,
            MessagePayload::Ping,
        ))
        .await
        .is_ok();

    let scheduler = ipc
        .request(Message::new(
            IPC_SENDER_HUB,
            ProcessType::Scheduler,
            MessagePayload::Ping,
        ))
        .await
        .is_ok();

    Ok(Json(HealthResponse { storage, scheduler }))
}

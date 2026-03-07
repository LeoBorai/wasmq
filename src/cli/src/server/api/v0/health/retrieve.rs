use axum::{Extension, Json};
use serde::Serialize;

use mate_ipc::protocol::{Message, ProcessType, SystemMessage};

use crate::process::hub::IPC_SENDER_HUB;
use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

#[derive(Serialize)]
pub struct HealthResponse {
    pub executors: usize,
    pub storage: bool,
    pub scheduler: bool,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
) -> Result<Json<HealthResponse>, ApiError> {
    let executors = services.config.executors.count;
    let ipc = services.hub.ipc();
    let storage = ipc
        .request(Message::new(
            IPC_SENDER_HUB,
            ProcessType::Storage,
            SystemMessage::Ping,
        ))
        .await
        .is_ok();

    let scheduler = ipc
        .request(Message::new(
            IPC_SENDER_HUB,
            ProcessType::Scheduler,
            SystemMessage::Ping,
        ))
        .await
        .is_ok();

    let mut active_executors = 0;

    for i in 0..executors {
        if ipc
            .request(Message::new(
                IPC_SENDER_HUB,
                ProcessType::Executor(i),
                SystemMessage::Ping,
            ))
            .await
            .is_ok()
        {
            active_executors += 1;
        }
    }

    Ok(Json(HealthResponse {
        storage,
        scheduler,
        executors: active_executors,
    }))
}

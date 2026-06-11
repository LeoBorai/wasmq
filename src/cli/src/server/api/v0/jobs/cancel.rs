use axum::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use ulid::Ulid;

use mate_ipc::protocol::{Message, MessagePayload, ProcessType};

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Path(id): Path<Ulid>,
) -> Result<StatusCode, ApiError> {
    let response = services
        .hub
        .ipc()
        .request(Message::new(
            ProcessType::Hub,
            ProcessType::Storage,
            MessagePayload::CancelJob(id),
        ))
        .await
        .map_err(|err| ApiError::new(err.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    match response.payload {
        MessagePayload::JobUpdated(Ok(())) => Ok(StatusCode::NO_CONTENT),
        MessagePayload::JobUpdated(Err(message)) => {
            Err(ApiError::new(message, StatusCode::CONFLICT))
        }
        _ => Err(ApiError::new(
            "Unexpected response from storage service".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )),
    }
}

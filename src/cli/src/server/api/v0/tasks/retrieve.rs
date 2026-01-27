use axum::http::StatusCode;
use axum::{Extension, Json};

use mate::proto::task::TaskIdentifier;

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

pub async fn handler(
    Extension(services): Extension<SharedServices>,
) -> Result<Json<Vec<TaskIdentifier>>, ApiError> {
    let tasks = services.repo.list().await.map_err(|err| ApiError {
        message: err.to_string(),
        status: StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(Json(tasks))
}

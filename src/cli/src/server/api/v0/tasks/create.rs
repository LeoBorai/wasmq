use std::str::FromStr;

use axum::Extension;
use axum::extract::{Multipart, Path};
use axum::http::StatusCode;
use bytes::Bytes;

use mate::proto::task::TaskIdentifier;

use crate::server::api::v0::ApiError;
use crate::server::state::SharedServices;

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Path((namespace, name, version)): Path<(String, String, String)>,
    mut multipart: Multipart,
) -> Result<(), ApiError> {
    let id = TaskIdentifier::from_str(&format!("{}/{}@{}", namespace, name, version)).map_err(
        |err| ApiError {
            message: err.to_string(),
            status: StatusCode::BAD_GATEWAY,
        },
    )?;
    let wasm = extract_file_from_multipart(&mut multipart, "task").await?;

    services
        .repo
        .store(&id, wasm)
        .await
        .map_err(|err| ApiError {
            message: err.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(())
}

async fn extract_file_from_multipart(
    multipart: &mut Multipart,
    field_name: &str,
) -> Result<Bytes, ApiError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| ApiError {
        message: e.to_string(),
        status: StatusCode::BAD_REQUEST,
    })? {
        if field.name() == Some(field_name) {
            let data = field.bytes().await.map_err(|e| ApiError {
                message: e.to_string(),
                status: StatusCode::BAD_REQUEST,
            })?;
            return Ok(data);
        }
    }

    Err(ApiError {
        message: format!("Field '{}' not found in multipart data", field_name),
        status: StatusCode::BAD_REQUEST,
    })
}

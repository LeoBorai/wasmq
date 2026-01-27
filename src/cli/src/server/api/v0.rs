pub mod jobs;
pub mod tasks;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new()
        .nest("/jobs", jobs::routes())
        .nest("/tasks", tasks::routes())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiError {
    pub message: String,
    #[serde(skip)]
    pub status: StatusCode,
}

impl ApiError {
    pub fn new(message: String, status: StatusCode) -> Self {
        Self { message, status }
    }

    pub fn five_hundred() -> Self {
        Self::new(
            "Internal server error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        if let Ok(status) = StatusCode::from_u16(self.status.as_u16()) {
            let mut response = Json(self).into_response();

            *response.status_mut() = status;
            return response;
        }

        ApiError::five_hundred().into_response()
    }
}

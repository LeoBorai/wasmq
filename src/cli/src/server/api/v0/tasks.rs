mod create;
mod retrieve;

use axum::routing::{Router, get, post};

pub fn routes() -> Router {
    Router::new()
        .route("/{namespace}/{name}/{version}", post(create::handler))
        .route("/", get(retrieve::handler))
}

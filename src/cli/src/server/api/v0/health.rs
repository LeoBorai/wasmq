mod retrieve;

use axum::routing::{Router, get};

pub fn routes() -> Router {
    Router::new().route("/", get(retrieve::handler))
}

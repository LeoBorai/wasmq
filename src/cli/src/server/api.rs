pub mod v0;

use axum::{Extension, Router};

use crate::server::state::SharedServices;

pub fn routes(services: SharedServices) -> Router {
    Router::new()
        .nest("/api/v0", v0::routes())
        .layer(Extension(services))
}

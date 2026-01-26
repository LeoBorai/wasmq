mod api;
mod state;

use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use mate_config::Config;

use crate::process::hub::Hub;
use crate::server::api::routes;
use crate::server::state::Services;
use crate::utils::shutdown_signal;

pub async fn run_server(config: &Config, hub: Arc<Hub>) -> Result<()> {
    let services = Arc::new(Services::new(hub));
    let app = Router::new().merge(routes(services));
    let listener = TcpListener::bind("0.0.0.0:6283").await?;

    info!(addr=?config.hub.api_addr, "Server listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

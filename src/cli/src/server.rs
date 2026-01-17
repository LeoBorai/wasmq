mod api;
mod state;

use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use mate_ipc::channel::IpcServer;

use crate::server::api::routes;
use crate::server::state::Services;

pub async fn run_server(ipc_server: Arc<IpcServer>) -> Result<()> {
    let services = Arc::new(Services::new(ipc_server));
    let app = Router::new().merge(routes(services));
    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    println!("Server listening...");

    axum::serve(listener, app).await?;
    Ok(())
}

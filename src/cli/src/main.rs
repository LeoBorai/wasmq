mod cli;
mod process;
mod server;
mod transport;
mod utils;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let env_layer = EnvFilter::try_from_default_env()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_layer)
        .init();

    let cli = Cli::parse();
    cli.exec().await?;

    Ok(())
}

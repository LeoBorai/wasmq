mod cli;
mod process;
mod server;
mod transport;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.exec().await?;
    Ok(())
}

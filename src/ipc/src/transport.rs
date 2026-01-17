pub mod unix_socket;

use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::protocol::Message;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TransportConfig {
    UnixSocket { base_path: PathBuf },
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, msg: Message) -> Result<()>;
    async fn recv(&mut self) -> Result<Message>;
    async fn request(&mut self, msg: Message) -> Result<Message>;
    async fn close(&mut self) -> Result<()>;
}

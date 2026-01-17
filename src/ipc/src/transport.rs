pub mod unix_socket;

use anyhow::Result;
use async_trait::async_trait;

use crate::protocol::Message;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, msg: Message) -> Result<()>;
    async fn recv(&self) -> Result<Message>;
    async fn request(&self, msg: Message) -> Result<Message>;
    async fn close(&mut self) -> Result<()>;
}

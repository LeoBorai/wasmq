use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::protocol::{Message, ProcessType};
use crate::transport::Transport;

/// High-level IPC server that wraps any Transport implementation
pub struct IpcServer {
    process_type: ProcessType,
    transport: Arc<Box<dyn Transport>>,
    message_tx: UnboundedSender<Message>,
    message_rx: Arc<Mutex<UnboundedReceiver<Message>>>,
}

impl IpcServer {
    pub fn new(process_type: ProcessType, transport: Box<dyn Transport>) -> Self {
        let (message_tx, message_rx) = unbounded_channel();
        let transport = Arc::new(transport);
        let message_rx = Arc::new(Mutex::new(message_rx));

        Self {
            process_type,
            transport,
            message_tx,
            message_rx,
        }
    }

    /// Start listening for incoming messages
    /// This runs the transport's receive loop and forwards messages to the internal channel
    pub async fn listen(&self) -> Result<()> {
        let tx = self.message_tx.clone();

        loop {
            match self.transport.recv().await {
                Ok(msg) => {
                    if tx.send(msg).is_err() {
                        // Channel closed, exit loop
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Transport receive error for {:?}: {}", self.process_type, e);
                    // Continue receiving despite errors
                }
            }
        }

        Ok(())
    }

    /// Send a message without waiting for response
    pub async fn send(&self, msg: Message) -> Result<()> {
        self.transport.send(msg).await
    }

    /// Send a message and wait for response (request-response pattern)
    pub async fn request(&self, msg: Message) -> Result<Message> {
        self.transport.request(msg).await
    }

    /// Get receiver for incoming messages
    pub async fn receiver(&self) -> Arc<Mutex<UnboundedReceiver<Message>>> {
        Arc::clone(&self.message_rx)
    }

    /// Get the process type this server represents
    pub fn process_type(&self) -> ProcessType {
        self.process_type
    }

    /// Gracefully shutdown the server
    pub async fn shutdown(&mut self) -> Result<()> {
        self.transport.close().await
    }
}

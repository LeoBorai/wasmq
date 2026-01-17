use std::collections::HashMap;
use std::fs::remove_file;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::sync::oneshot::{Sender, channel};
use tokio::time::timeout;
use uuid::Uuid;

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

use crate::protocol::{Message, ProcessType};

pub struct IpcServer {
    socket_path: PathBuf,
    message_tx: UnboundedSender<Message>,
    message_rx: UnboundedReceiver<Message>,
    pending_responses: HashMap<Uuid, Sender<Message>>,
}

impl IpcServer {
    pub async fn new(process_type: ProcessType) -> Result<Self> {
        let socket_path = PathBuf::from(format!("/tmp/mate_system_{:?}.sock", process_type));
        let _ = remove_file(&socket_path);
        let (message_tx, message_rx) = unbounded_channel();

        Ok(Self {
            socket_path,
            message_tx,
            message_rx,
            pending_responses: HashMap::new(),
        })
    }

    pub async fn listen(&mut self) -> Result<()> {
        let listener = UnixListener::bind(&self.socket_path)?;

        loop {
            tokio::select! {
                Ok((stream, _)) = listener.accept() => {
                    let tx = self.message_tx.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, tx).await {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                }
                Some(msg) = self.message_rx.recv() => {
                    if let Some(reply_to) = msg.reply_to
                        && let Some(sender) = self.pending_responses.remove(&reply_to) {
                            let _ = sender.send(msg);
                        }
                }
            }
        }
    }

    pub async fn send_message(&self, msg: &Message) -> Result<()> {
        let target_socket = PathBuf::from(format!("/tmp/job_system_{:?}.sock", msg.to));
        let mut stream = UnixStream::connect(target_socket).await?;

        let serialized = serde_json::to_vec(msg)?;
        let len = (serialized.len() as u32).to_le_bytes();

        stream.write_all(&len).await?;
        stream.write_all(&serialized).await?;
        stream.flush().await?;

        Ok(())
    }

    pub async fn request(&mut self, msg: &Message) -> Result<Message> {
        let (tx, rx) = channel();
        self.pending_responses.insert(msg.id, tx);

        self.send_message(msg).await?;

        timeout(REQUEST_TIMEOUT, rx)
            .await?
            .with_context(|| format!("Request timed out for message with ID. {}", msg.id))
    }

    pub fn receiver(&mut self) -> &mut UnboundedReceiver<Message> {
        &mut self.message_rx
    }
}

async fn handle_connection(mut stream: UnixStream, tx: UnboundedSender<Message>) -> Result<()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_le_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;

    let message: Message = serde_json::from_slice(&buf)?;
    tx.send(message)?;

    Ok(())
}

use std::sync::Arc;

use anyhow::{Result, bail};

use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Message, MessagePayload, ProcessType};
use mate_ipc::transport::Transport;

pub struct ExecutorProcess {
    id: usize,
    ipc: Arc<IpcServer>,
}

impl ExecutorProcess {
    pub fn new(transport: Box<dyn Transport>, id: usize) -> Self {
        let ipc = Arc::new(IpcServer::new(ProcessType::Executor(id), transport));

        Self { id, ipc }
    }

    pub async fn run(&mut self) -> Result<()> {
        tokio::select! {
            Err(err) = self.message_consumer() => {
                bail!("Storage message consumer failed. {:#?}", err);
            }
        }
    }

    async fn message_consumer(&self) -> Result<()> {
        let ipc_clone = Arc::clone(&self.ipc);

        tokio::spawn(async move {
            let _ = ipc_clone.listen().await;
        });

        let rx = self.ipc.receiver().await;
        let mut rx = rx.lock().await;

        while let Some(msg) = rx.recv().await {
            if let Some(response) = self.handle_message(msg.clone()).await {
                let response_msg = Message::new(ProcessType::Executor(self.id), msg.from, response)
                    .reply_to(msg.id)
                    .to_owned();

                if let Err(err) = self.ipc.send(response_msg).await {
                    eprintln!(
                        "Error while sending message from Storage to IPC. {:#?}",
                        err
                    );
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> Option<MessagePayload> {
        match msg.payload {
            MessagePayload::Ping => Some(MessagePayload::Pong),
            MessagePayload::Shutdown => Some(MessagePayload::ShutdownAck),
            _ => None,
        }
    }
}

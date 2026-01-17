use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Job, Message, MessagePayload, ProcessType};
use mate_ipc::transport::Transport;

pub struct StorageProcess {
    ipc: Arc<IpcServer>,
    jobs: HashMap<Uuid, Job>,
}

impl StorageProcess {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let ipc = Arc::new(IpcServer::new(ProcessType::Storage, transport));

        Self {
            jobs: HashMap::new(),
            ipc,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let ipc_clone = Arc::clone(&self.ipc);

        tokio::spawn(async move {
            let _ = ipc_clone.listen().await;
        });

        // Process messages
        let rx = self.ipc.receiver().await;
        let mut rx = rx.lock().await;

        while let Some(msg) = rx.recv().await {
            if let Some(response) = self.handle_message(msg.clone()).await {
                let response_msg = Message {
                    id: Uuid::new_v4(),
                    from: ProcessType::Storage,
                    to: msg.from,
                    payload: response,
                    reply_to: Some(msg.id),
                };

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

    async fn handle_message(&mut self, msg: Message) -> Option<MessagePayload> {
        match msg.payload {
            MessagePayload::StoreJob(job) => {
                let id = job.id;
                self.jobs.insert(id, job.clone());
                Some(MessagePayload::JobStored(Ok(job)))
            }
            MessagePayload::QueryJobs(query) => {
                let jobs: Vec<Job> = self
                    .jobs
                    .values()
                    .filter(|j| query.status.as_ref().is_none_or(|s| &j.status == s))
                    .cloned()
                    .collect();
                Some(MessagePayload::JobsResult(jobs))
            }
            _ => None,
        }
    }
}

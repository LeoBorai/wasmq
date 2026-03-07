mod backend;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use anyhow::{Result, bail};

use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{
    ExecutorMessage, HubMessage, Message, MessagePayload, ProcessType, SchedulerMessage,
    StorageMessage, SystemMessage,
};
use mate_ipc::transport::Transport;

use crate::backend::Backend;
use crate::backend::sqlite::SqliteBackend;

const IPC_SENDER_STORAGE: ProcessType = ProcessType::Storage;
const MAX_JOBS_PER_BATCH: usize = 5;

pub struct Storage {
    ipc: Arc<IpcServer>,
    backend: Arc<dyn Backend>,
}

impl Storage {
    pub async fn new(transport: Box<dyn Transport>, home: PathBuf) -> Result<Self> {
        let ipc = Arc::new(IpcServer::new(IPC_SENDER_STORAGE, transport));
        let home = home.join("storage.sqlite");
        let backend = Arc::new(SqliteBackend::new(home.to_str().unwrap()).await?);

        Ok(Self { ipc, backend })
    }

    pub async fn run(&mut self) -> Result<()> {
        tokio::select! {
            Err(err) = self.message_consumer() => {
                bail!("Storage message consumer failed. {:#?}", err);
            }
        }
    }

    async fn message_consumer(&mut self) -> Result<()> {
        let ipc_clone = Arc::clone(&self.ipc);

        tokio::spawn(async move {
            let _ = ipc_clone.listen().await;
        });

        let rx = self.ipc.receiver().await;
        let mut rx = rx.lock().await;

        while let Some(msg) = rx.recv().await {
            if let Some(response) = self.handle_message(msg.clone()).await {
                let response_msg = Message::new(IPC_SENDER_STORAGE, msg.from, response)
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

    async fn handle_message(&mut self, msg: Message) -> Option<MessagePayload> {
        match msg.payload {
            MessagePayload::Hub(HubMessage::StoreJob(job)) => {
                let job_id = job.id;
                match self.backend.create_job(*job).await {
                    Ok(job) => Some(StorageMessage::JobStored(Box::new(Ok(job))).into()),
                    Err(err) => Some(
                        StorageMessage::JobStored(Box::new(Err(format!(
                            "Failed to store job {}: {err}",
                            job_id
                        ))))
                        .into(),
                    ),
                }
            }
            MessagePayload::Hub(HubMessage::QueryJobs(query)) => {
                match self.backend.retrieve_jobs(query).await {
                    Ok(jobs) => Some(StorageMessage::JobsResult(jobs).into()),
                    Err(_) => Some(StorageMessage::JobsResult(vec![]).into()),
                }
            }
            MessagePayload::Executor(ExecutorMessage::JobCompleted(id, result)) => {
                match self.backend.update_job_completed(id, result).await {
                    Ok(_) => Some(StorageMessage::JobUpdated(Ok(())).into()),
                    Err(err) => Some(
                        StorageMessage::JobUpdated(Err(format!(
                            "Failed to update completion status for job {id}: {err}"
                        )))
                        .into(),
                    ),
                }
            }
            MessagePayload::Scheduler(SchedulerMessage::ClaimJobs((_, end))) => {
                match self
                    .backend
                    .claim_jobs(MAX_JOBS_PER_BATCH, SystemTime::now(), end)
                    .await
                {
                    Ok(jobs) => Some(StorageMessage::JobsResult(jobs).into()),
                    Err(_) => Some(StorageMessage::JobsResult(vec![]).into()),
                }
            }
            MessagePayload::System(SystemMessage::Ping) => Some(SystemMessage::Pong.into()),
            MessagePayload::System(SystemMessage::Shutdown) => {
                Some(SystemMessage::ShutdownAck.into())
            }
            MessagePayload::Hub(HubMessage::UpdateJobStatus(update)) => {
                Some(
                    StorageMessage::JobUpdated(Err(format!(
                        "Job status update ignored by storage backend for update: {:?}",
                        update
                    )))
                    .into(),
                )
            }
            _ => None,
        }
    }
}

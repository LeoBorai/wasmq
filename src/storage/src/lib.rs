mod backend;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Result, bail};
use tokio::sync::Mutex;
use tracing::debug;

use mate::proto::job::{Job, JobResult, JobStatus};
use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Message, MessagePayload, ProcessType};
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
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let ipc = Arc::new(IpcServer::new(IPC_SENDER_STORAGE, transport));
        let backend = Arc::new(SqliteBackend::new());

        Self { ipc, backend }
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
            MessagePayload::JobCompleted(id, result) => {
                match self.backend.update_job_completed(id, result).await {
                    Ok(job) => Some(MessagePayload::JobUpdated(Ok(()))),
                    Err(err) => Some(MessagePayload::JobUpdated(Err(format!(
                        "Failed to update completion status for job {id}: {err}"
                    )))),
                }
            }
            MessagePayload::StoreJob(job) => {
                match self.backend.create_job(job).await {
                    Ok(job) => Some(MessagePayload::JobStored(Ok(job))),
                    Err(err) => Some(MessagePayload::JobStored(Err(format!(
                        "Failed to store job {}: {err}",
                        job.id
                    )))),
                }
            }
            MessagePayload::QueryJobs(query) => {
                let jobs = self.jobs.lock().await;
                let jobs_clone = jobs.clone();
                drop(jobs);
                let jobs: Vec<Job> = jobs_clone
                    .values()
                    .filter(|j| query.status.as_ref().is_none_or(|s| &j.status == s))
                    .filter(|j| {
                        query.time_range.as_ref().is_none_or(|tr| {
                            j.scheduled_at >= tr.0 - Duration::from_secs(1)
                                && j.scheduled_at <= tr.1 + Duration::from_secs(1)
                        })
                    })
                    .cloned()
                    .collect();
                Some(MessagePayload::JobsResult(jobs))
            }
            MessagePayload::ClaimJobs((_, end)) => {
                let mut jobs = self.jobs.lock().await;
                let jobs: Vec<Job> = jobs
                    .iter_mut()
                    .filter(|(_, j)| j.status == JobStatus::Scheduled)
                    .filter(|(_, j)| j.scheduled_at <= end)
                    .take(MAX_JOBS_PER_BATCH)
                    .map(|(_, job)| {
                        job.status = JobStatus::Claimed;
                        job.clone()
                    })
                    .collect();
                Some(MessagePayload::JobsResult(jobs))
            }
            MessagePayload::Ping => Some(MessagePayload::Pong),
            MessagePayload::Shutdown => Some(MessagePayload::ShutdownAck),
            _ => None,
        }
    }
}

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Result, bail};
use tracing::{debug, error, info};
use uuid::Uuid;

use mate::proto::job::{Job, JobResult, JobStatus};
use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Message, MessagePayload, ProcessType};
use mate_ipc::transport::Transport;

const IPC_SENDER_STORAGE: ProcessType = ProcessType::Storage;

pub struct StorageProcess {
    ipc: Arc<IpcServer>,
    jobs: HashMap<Uuid, Job>,
}

impl StorageProcess {
    pub fn new(transport: Box<dyn Transport>) -> Self {
        let ipc = Arc::new(IpcServer::new(IPC_SENDER_STORAGE, transport));

        Self {
            jobs: HashMap::new(),
            ipc,
        }
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
                debug!(%id, ?result, "Job completed");

                if let Some(job) = self.jobs.get_mut(&id) {
                    job.completed_at = Some(SystemTime::now());

                    match &result {
                        JobResult::Success(_) => {
                            job.status = JobStatus::Completed;
                        }
                        JobResult::Failure(err) => {
                            if job.retry_count < job.max_retries {
                                job.status = JobStatus::Scheduled;
                                job.retry_count += 1;
                                job.errors.push(err.to_string());
                            } else {
                                job.status = JobStatus::Failed;
                            }
                        }
                    }

                    job.result = Some(result);

                    return Some(MessagePayload::JobStored(Ok(job.clone())));
                }

                Some(MessagePayload::JobUpdated(Err(format!(
                    "Failed to update completion status for job {id}: job not found in storage"
                ))))
            }
            MessagePayload::StoreJob(job) => {
                let id = job.id;
                self.jobs.insert(id, job.clone());

                if let Err(err) = self
                    .ipc
                    .send(Message::new(
                        ProcessType::Storage,
                        ProcessType::Scheduler,
                        MessagePayload::JobStored(Ok(job.clone())),
                    ))
                    .await
                {
                    error!(?err, "Failed to notify Scheduler about stored Job {id}");
                }

                Some(MessagePayload::JobStored(Ok(job)))
            }
            MessagePayload::QueryJobs(query) => {
                let jobs: Vec<Job> = self
                    .jobs
                    .values()
                    .filter(|j| {
                        query.time_range.as_ref().is_none_or(|tr| {
                            j.scheduled_at >= tr.0 - Duration::from_secs(1)
                                && j.scheduled_at <= tr.1 + Duration::from_secs(1)
                        })
                    })
                    .filter(|j| query.status.as_ref().is_none_or(|s| &j.status == s))
                    .cloned()
                    .collect();
                info!(jobs = jobs.len(), "Returning jobs from storage");
                Some(MessagePayload::JobsResult(jobs))
            }
            MessagePayload::QueryScheduledJobs(sys_time) => {
                let jobs: Vec<Job> = self
                    .jobs
                    .values()
                    .filter(|j| j.status == JobStatus::Scheduled)
                    .filter(|j| j.scheduled_at <= sys_time)
                    .cloned()
                    .collect();
                Some(MessagePayload::JobsResult(jobs))
            }
            MessagePayload::Ping => Some(MessagePayload::Pong),
            MessagePayload::Shutdown => Some(MessagePayload::ShutdownAck),
            _ => None,
        }
    }
}

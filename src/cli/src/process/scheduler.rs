use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Result, bail};
use tokio::sync::Mutex;
use tokio::time::interval;
use tracing::{error, warn};
use uuid::Uuid;

use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Job, JobStatus, Message, MessagePayload, ProcessType};
use mate_ipc::transport::Transport;

const IPC_SENDER_SCHEDULER: ProcessType = ProcessType::Scheduler;

pub struct SchedulerProcess {
    ipc: Arc<IpcServer>,
    executor_count: usize,
    current_executor: Mutex<usize>,
}

impl SchedulerProcess {
    pub async fn new(transport: Box<dyn Transport>, executor_count: usize) -> Result<Self> {
        let ipc = Arc::new(IpcServer::new(IPC_SENDER_SCHEDULER, transport));

        Ok(Self {
            ipc,
            executor_count,
            current_executor: Mutex::new(0),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tokio::select! {
            Err(err) = self.clock() => {
                error!(?err, "Scheduler clock failed.");
            }
            Err(err) = self.message_consumer() => {
                error!(?err, "Scheduler message consumer failed.");
            }
        }

        Ok(())
    }

    async fn clock(&self) -> Result<()> {
        let mut interval = interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            self.check_and_schedule_jobs().await?;
        }
    }

    async fn check_and_schedule_jobs(&self) -> Result<()> {
        // Query storage for Scheduled jobs
        let query_msg = Message {
            id: Uuid::new_v4(),
            from: IPC_SENDER_SCHEDULER,
            to: ProcessType::Storage,
            payload: MessagePayload::QueryScheduledJobs(SystemTime::now()),
            reply_to: None,
        };

        match self.ipc.request(query_msg).await {
            Ok(response) => {
                if let MessagePayload::JobsResult(jobs) = response.payload {
                    for job in jobs {
                        if let Err(err) = self.dispatch_job(job).await {
                            warn!(?err, "Failed to dispatch job");
                        }
                    }
                }
            }
            Err(err) => {
                error!(?err, "Failed to query scheduled jobs from Storage");
            }
        }

        Ok(())
    }

    /// Dispatch Jobs by performing Round-Robin distribution on available
    /// executor processes and sending a [`Message`] to the next executor.
    async fn dispatch_job(&self, job: Job) -> Result<()> {
        let mut current_executor = self.current_executor.lock().await;
        let executor_id = *current_executor;
        *current_executor = (*current_executor + 1) % self.executor_count;
        drop(current_executor);
        let job_id = job.id;

        let msg = Message::new(
            IPC_SENDER_SCHEDULER,
            ProcessType::Executor(executor_id),
            MessagePayload::ExecuteJob(job),
        );

        if let Err(err) = self.ipc.send(msg).await {
            bail!("Failed to send message to dispatch job via IPC. {:?}", err);
        }

        if let Err(err) = self
            .ipc
            .send(Message::new(
                IPC_SENDER_SCHEDULER,
                ProcessType::Storage,
                MessagePayload::UpdateJobStatus(job_id, JobStatus::Pending),
            ))
            .await
        {
            error!(
                ?err,
                "Failed to send message to Storage in order to update Job status"
            );
        }

        Ok(())
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
                let response_msg = Message::new(ProcessType::Storage, msg.from, response)
                    .reply_to(msg.id)
                    .to_owned();

                if let Err(err) = self.ipc.send(response_msg).await {
                    eprintln!(
                        "Error while sending message from Scheduler to IPC. {:#?}",
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

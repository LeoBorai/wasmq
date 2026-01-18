use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::Result;
use tokio::sync::Mutex;
use tokio::time::interval;
use tracing::{error, info, warn};
use uuid::Uuid;

use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Job, Message, MessagePayload, ProcessType};
use mate_ipc::transport::Transport;

pub struct SchedulerProcess {
    ipc: Arc<IpcServer>,
    executor_count: usize,
    current_executor: Mutex<usize>,
}

impl SchedulerProcess {
    pub async fn new(transport: Box<dyn Transport>, executor_count: usize) -> Result<Self> {
        let ipc = Arc::new(IpcServer::new(ProcessType::Storage, transport));

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
        // Query storage for pending jobs
        let query_msg = Message {
            id: Uuid::new_v4(),
            from: ProcessType::Scheduler,
            to: ProcessType::Storage,
            payload: MessagePayload::QueryPendingJobs(SystemTime::now()),
            reply_to: None,
        };

        let response = self.ipc.request(query_msg).await?;

        info!(?response, "Got response in scheduler");

        if let MessagePayload::JobsResult(jobs) = response.payload {
            for job in jobs {
                self.dispatch_job(job).await?;
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

        let msg = Message::new(
            ProcessType::Scheduler,
            ProcessType::Executor(executor_id),
            MessagePayload::ExecuteJob(job),
        );

        if let Err(err) = self.ipc.request(msg).await {
            warn!(?err, "Failed te send message");
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

use std::collections::BinaryHeap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::{Result, bail};
use tokio::sync::Mutex;
use tokio::time::{interval, sleep};
use tracing::{debug, error, warn};

use mate::proto::job::{Job, JobStatus};
use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{
    HubMessage, Message, MessagePayload, ProcessType, SchedulerMessage, StorageMessage,
    SystemMessage,
};
use mate_ipc::transport::Transport;

const IPC_SENDER_SCHEDULER: ProcessType = ProcessType::Scheduler;
const CHECK_INTERVAL: Duration = Duration::from_secs(10);
const LOOKAHEAD_WINDOW: Duration = Duration::from_mins(5);
const PERIODIC_RELOAD: Duration = Duration::from_secs(30);
const SLEEP_INTERVAL: Duration = Duration::from_secs(1);

pub struct Scheduler {
    ipc: Arc<IpcServer>,
    executor_count: usize,
    current_executor: Mutex<usize>,
    queue: Arc<Mutex<BinaryHeap<Job>>>,
}

impl Scheduler {
    pub async fn new(transport: Box<dyn Transport>, executor_count: usize) -> Result<Self> {
        let ipc = Arc::new(IpcServer::new(IPC_SENDER_SCHEDULER, transport));
        let queue = Arc::new(Mutex::new(BinaryHeap::new()));

        Ok(Self {
            ipc,
            executor_count,
            current_executor: Mutex::new(0),
            queue,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tokio::select! {
            Err(err) = self.scheduler() => {
                error!(?err, "Scheduler clock failed.");
            }
            Err(err) = self.message_consumer() => {
                error!(?err, "Scheduler message consumer failed.");
            }
            Err(err) = self.periodic_reload() => {
                error!(?err, "Periodic reload failed.");
            }
        }

        Ok(())
    }

    async fn scheduler(&self) -> Result<()> {
        // preload jobs on startup into our queue
        self.load_upcoming_jobs().await?;

        loop {
            let mut queue = self.queue.lock().await;
            let job = match queue.peek() {
                Some(job) => job.clone(),
                None => {
                    drop(queue);
                    sleep(CHECK_INTERVAL).await;
                    self.load_upcoming_jobs().await?;
                    continue;
                }
            };
            let now = SystemTime::now();
            let time_until_job = match job.scheduled_at.duration_since(now) {
                Ok(duration) => duration,
                Err(_) => {
                    if let Some(job) = queue.pop() {
                        drop(queue);
                        debug!("Executing overdue job: {:?}", job.id);
                        if let Err(err) = self.dispatch_job(job).await {
                            warn!(?err, "Failed to dispatch job");
                        }
                    }

                    continue;
                }
            };

            drop(queue);

            let sleep_duration = time_until_job.min(CHECK_INTERVAL);
            let sleep_duration = sleep_duration.max(SLEEP_INTERVAL);
            sleep(sleep_duration).await;
        }
    }

    async fn periodic_reload(&self) -> Result<()> {
        let mut interval = interval(PERIODIC_RELOAD);

        loop {
            interval.tick().await;

            if let Err(err) = self.load_upcoming_jobs().await {
                warn!(?err, "Failed to reload jobs from storage");
            }
        }
    }

    /// Fetches upcoming jobs from Storage and loads them into the queue
    async fn load_upcoming_jobs(&self) -> Result<()> {
        let now = SystemTime::now();
        let future = now + LOOKAHEAD_WINDOW;
        let request = Message::new(
            IPC_SENDER_SCHEDULER,
            ProcessType::Storage,
            SchedulerMessage::ClaimJobs((now, future)),
        );

        match self.ipc.request(request).await {
            Ok(response) => {
                if let MessagePayload::Storage(StorageMessage::JobsResult(jobs)) = response.payload
                {
                    let mut queue = self.queue.lock().await;

                    for job in jobs {
                        if queue.iter().any(|j| j.id == job.id) {
                            continue;
                        }

                        queue.push(job);
                    }
                }
            }
            Err(err) => {
                error!(?err, "Failed to load jobs from Storage");
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

        self.ipc
            .send(Message::new(
                IPC_SENDER_SCHEDULER,
                ProcessType::Storage,
                MessagePayload::Hub(HubMessage::UpdateJobStatus(job_id, JobStatus::Pending)),
            ))
            .await?;

        let msg = Message::new(
            IPC_SENDER_SCHEDULER,
            ProcessType::Executor(executor_id),
            SchedulerMessage::ExecuteJob(Box::new(job)),
        );

        if let Err(err) = self.ipc.send(msg).await {
            // If dispatch fails, revert job status to `Scheduled`
            self.ipc
                .send(Message::new(
                    IPC_SENDER_SCHEDULER,
                    ProcessType::Storage,
                    MessagePayload::Hub(HubMessage::UpdateJobStatus(job_id, JobStatus::Scheduled)),
                ))
                .await?;
            bail!("Failed to send message to dispatch job via IPC. {:?}", err);
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
            MessagePayload::System(message) => match message {
                SystemMessage::Ping => Some(SystemMessage::Pong.into()),
                SystemMessage::Shutdown => Some(SystemMessage::ShutdownAck.into()),
                _ => None,
            },
            _ => None,
        }
    }
}

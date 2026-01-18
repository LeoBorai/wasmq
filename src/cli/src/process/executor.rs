use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use bytes::Bytes;
use mate_config::Config;
use tokio::sync::RwLock;
use tokio::{fs, task};
use tracing::{error, info};

use mate_executor::Executor;
use mate_ipc::channel::IpcServer;
use mate_ipc::protocol::{Job, JobResult, Message, MessagePayload, ProcessType};
use mate_ipc::transport::Transport;

pub struct ExecutorProcess {
    id: usize,
    ipc: Arc<IpcServer>,
    executor: Arc<Executor>,
    modules: Arc<RwLock<HashMap<String, Bytes>>>,
    config: Config,
}

impl ExecutorProcess {
    pub fn new(transport: Box<dyn Transport>, config: Config, id: usize) -> Result<Self> {
        let modules = Arc::new(RwLock::new(HashMap::new()));
        let executor = Arc::new(Executor::new());
        let ipc = Arc::new(IpcServer::new(ProcessType::Executor(id), transport));

        Ok(Self {
            id,
            ipc,
            executor,
            modules,
            config,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        tokio::select! {
            Err(err) = self.message_consumer() => {
                bail!("Executor message consumer failed. {:#?}", err);
            }
        }
    }

    /// Load module on-demand with caching
    async fn get_or_load_module(&self, task_name: &str) -> Result<Bytes> {
        {
            let modules = self.modules.read().await;
            if let Some(module) = modules.get(task_name) {
                return Ok(module.clone());
            }
        }

        info!(task_name, "Loading WASM module");

        let path = self
            .config
            .registry
            .path
            .join(format!("{}.wasm", task_name));
        let wasm = fs::read(&path)
            .await
            .context(format!("Failed to read WASM file: {:?}", path))?;
        let wasm: Bytes = wasm.into();

        {
            let mut modules = self.modules.write().await;
            modules.insert(task_name.to_string(), wasm.clone());
        }

        info!(task_name, "WASM module loaded and cached");
        Ok(wasm)
    }

    pub async fn execute(&self, job: Job) -> Result<()> {
        let ipc = Arc::clone(&self.ipc);
        let executor = Arc::clone(&self.executor);
        let task_name = job.task.clone();
        let job_id = job.id;
        let payload = job.payload.clone();
        let process_type = self.process_type();
        let task = match self.get_or_load_module(&task_name).await {
            Ok(m) => m,
            Err(err) => {
                error!(?err, task_name, "Failed to load WASM Task");

                if let Err(err) = ipc
                    .request(Message::new(
                        self.process_type(),
                        ProcessType::Storage,
                        MessagePayload::JobCompleted(
                            job_id,
                            JobResult::Failure(format!("Task load failed: {}", err)),
                        ),
                    ))
                    .await
                {
                    error!(?err, "Failed to send message to Storage");
                }

                bail!("Failed to load module: {}", err);
            }
        };

        tokio::spawn(async move {
            info!(%job_id, %task_name, "Executing Job");

            let result = task::spawn(async move {
                executor
                    .run(task, payload.to_string().as_bytes().to_vec().into())
                    .await
            })
            .await;

            let job_result = match result {
                Ok(Ok(output)) => {
                    info!(%job_id, ?output, "Job completed successfully");
                    JobResult::Success(output)
                }
                Ok(Err(err)) => {
                    error!(?err, %job_id, "Job execution failed");
                    JobResult::Failure(err.to_string())
                }
                Err(err) => {
                    error!(?err, %job_id, "Task panicked");
                    JobResult::Failure(format!("Task panicked: {}", err))
                }
            };

            if let Err(err) = ipc
                .request(Message::new(
                    process_type,
                    ProcessType::Storage,
                    MessagePayload::JobCompleted(job_id, job_result),
                ))
                .await
            {
                error!(?err, "Failed to send JobCompleted message to Storage");
            }
        });

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
                let response_msg = Message::new(ProcessType::Executor(self.id), msg.from, response)
                    .reply_to(msg.id)
                    .to_owned();

                if let Err(err) = self.ipc.send(response_msg).await {
                    eprintln!(
                        "Error while sending message from Executor to IPC. {:#?}",
                        err
                    );
                }
            }
        }

        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> Option<MessagePayload> {
        match msg.payload {
            MessagePayload::ExecuteJob(job) => match self.execute(job.clone()).await {
                Ok(()) => Some(MessagePayload::JobAccepted(job.id)),
                Err(err) => Some(MessagePayload::JobFailed(job.id, err.to_string())),
            },
            MessagePayload::Ping => Some(MessagePayload::Pong),
            MessagePayload::Shutdown => Some(MessagePayload::ShutdownAck),
            _ => None,
        }
    }

    fn process_type(&self) -> ProcessType {
        ProcessType::Executor(self.id)
    }
}

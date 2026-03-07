use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use mate::proto::job::{Job, JobQuery, JobResult, JobStatus};

pub type ExecutorId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HubMessage {
    StoreJob(Box<Job>),
    QueryJobs(JobQuery),
    UpdateJobStatus(Uuid, JobStatus),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageMessage {
    JobStored(Box<Result<Job, String>>),
    JobsResult(Vec<Job>),
    JobUpdated(Result<(), String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchedulerMessage {
    ClaimJobs((SystemTime, SystemTime)),
    ExecuteJob(Box<Job>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutorMessage {
    JobStarted(Uuid),
    JobCompleted(Uuid, JobResult),
    JobFailed(Uuid, String),
    JobAccepted(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemMessage {
    Ping,
    Pong,
    Shutdown,
    ShutdownAck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    Hub(HubMessage),
    Storage(StorageMessage),
    Scheduler(SchedulerMessage),
    Executor(ExecutorMessage),
    System(SystemMessage),
}

impl From<HubMessage> for MessagePayload {
    fn from(m: HubMessage) -> Self {
        MessagePayload::Hub(m)
    }
}

impl From<StorageMessage> for MessagePayload {
    fn from(m: StorageMessage) -> Self {
        MessagePayload::Storage(m)
    }
}

impl From<SchedulerMessage> for MessagePayload {
    fn from(m: SchedulerMessage) -> Self {
        MessagePayload::Scheduler(m)
    }
}

impl From<ExecutorMessage> for MessagePayload {
    fn from(m: ExecutorMessage) -> Self {
        MessagePayload::Executor(m)
    }
}

impl From<SystemMessage> for MessagePayload {
    fn from(m: SystemMessage) -> Self {
        MessagePayload::System(m)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProcessType {
    Hub,
    Executor(ExecutorId),
    Storage,
    Scheduler,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub from: ProcessType,
    pub to: ProcessType,
    pub payload: MessagePayload,
    pub reply_to: Option<Uuid>,
}

impl Message {
    pub fn new(from: ProcessType, to: ProcessType, payload: impl Into<MessagePayload>) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            payload: payload.into(),
            reply_to: None,
        }
    }

    pub fn reply_to(&mut self, id: Uuid) -> &mut Self {
        self.reply_to = Some(id);
        self
    }
}

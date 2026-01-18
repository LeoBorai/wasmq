use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub type ExecutorId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub payload: Value,
    pub status: JobStatus,
    pub scheduled_at: SystemTime,
    pub task: String,
    pub started_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
    pub result: Option<JobResult>,
    pub retry_count: u32,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobResult {
    Success(serde_json::Value),
    Failure(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobQuery {
    pub status: Option<JobStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    // Hub -> Storage
    StoreJob(Job),
    QueryJobs(JobQuery),
    UpdateJobStatus(Uuid, JobStatus),

    // Storage -> Hub/Scheduler/Executor
    JobStored(Result<Job, String>),
    JobsResult(Vec<Job>),
    JobUpdated(Result<(), String>),

    // Scheduler -> Storage
    QueryScheduledJobs(SystemTime),

    // Scheduler -> Executor
    ExecuteJob(Job),

    // Executor -> Storage
    JobStarted(Uuid),
    JobCompleted(Uuid, JobResult),
    JobFailed(Uuid, String),

    // Executor -> Scheduler (acknowledgment)
    JobAccepted(Uuid),

    // Health checks
    Ping,
    Pong,

    // Shutdown
    Shutdown,
    ShutdownAck,
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
    pub reply_to: Option<Uuid>, // For request-response pattern
}

impl Message {
    pub fn new(from: ProcessType, to: ProcessType, payload: MessagePayload) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            payload,
            reply_to: None,
        }
    }

    pub fn reply_to(&mut self, id: Uuid) -> &mut Self {
        self.reply_to = Some(id);
        self
    }
}

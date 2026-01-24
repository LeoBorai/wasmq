use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::proto::task::TaskIdentifier;

pub type ExecutorId = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub payload: Value,
    pub status: JobStatus,
    pub scheduled_at: SystemTime,
    pub task: TaskIdentifier,
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

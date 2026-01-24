use std::{fmt::Display, time::SystemTime};

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
    pub errors: Vec<String>,
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

impl Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            JobStatus::Pending => "Pending",
            JobStatus::Scheduled => "Scheduled",
            JobStatus::Running => "Running",
            JobStatus::Completed => "Completed",
            JobStatus::Failed => "Failed",
            JobStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobResult {
    Success(Value),
    Failure(String),
}

impl Display for JobResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobResult::Success(_) => write!(f, "Success"),
            JobResult::Failure(_) => write!(f, "Failure"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobQuery {
    pub status: Option<JobStatus>,
}

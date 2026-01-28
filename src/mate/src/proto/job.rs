use std::fmt::Display;
use std::time::SystemTime;
use std::{cmp::Ordering, str::FromStr};

use anyhow::{Error, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::proto::task::TaskIdentifier;

pub type ExecutorId = usize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub args: Value,
    pub status: JobStatus,
    pub scheduled_at: SystemTime,
    pub task: TaskIdentifier,
    pub started_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
    pub errors: Vec<String>,
    pub result: Option<JobResult>,
    pub attempts: u32,
    pub max_attempts: u32,
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        other.scheduled_at.cmp(&self.scheduled_at) // Earlier first
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Claimed,
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
            JobStatus::Claimed => "Claimed",
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

impl FromStr for JobStatus {
    type Err = Error;

    fn from_str(input: &str) -> Result<JobStatus, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "claimed" => Ok(JobStatus::Claimed),
            "pending" => Ok(JobStatus::Pending),
            "scheduled" => Ok(JobStatus::Scheduled),
            "running" => Ok(JobStatus::Running),
            "completed" => Ok(JobStatus::Completed),
            "failed" => Ok(JobStatus::Failed),
            "cancelled" => Ok(JobStatus::Cancelled),
            _ => bail!(
                "The value {} doesn't correspond to a valid JobStatus",
                input
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub time_range: Option<(SystemTime, SystemTime)>,
}

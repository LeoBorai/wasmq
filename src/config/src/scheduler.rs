use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchedulerConfig {
    pub check_interval_secs: u64,
}

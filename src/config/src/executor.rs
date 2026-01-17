use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutorConfig {
    pub count: usize,
    pub max_concurrent_jobs: usize,
}

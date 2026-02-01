use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutorConfig {
    pub count: usize,
    pub max_concurrent_jobs: usize,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            count: 1,
            max_concurrent_jobs: 5,
        }
    }
}

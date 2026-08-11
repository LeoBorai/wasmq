pub mod jobs;
pub mod tasks;

use std::sync::Arc;

use crate::client::HttpClient;
use crate::client::api::v0::jobs::ApiV0Jobs;
use crate::client::api::v0::tasks::ApiV0Tasks;

#[derive(Clone)]
pub struct V0 {
    pub jobs: ApiV0Jobs,
    pub tasks: ApiV0Tasks,
}

impl V0 {
    pub(super) fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            jobs: ApiV0Jobs::new(Arc::clone(&http_client)),
            tasks: ApiV0Tasks::new(Arc::clone(&http_client)),
        }
    }
}

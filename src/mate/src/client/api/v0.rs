pub mod jobs;

use std::sync::Arc;

use crate::client::HttpClient;
use crate::client::api::v0::jobs::ApiV0Jobs;

#[derive(Clone)]
pub struct V0 {
    pub jobs: ApiV0Jobs,
}

impl V0 {
    pub(super) fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            jobs: ApiV0Jobs::new(Arc::clone(&http_client)),
        }
    }
}

pub mod v0;

use std::sync::Arc;

use crate::client::HttpClient;
use crate::client::api::v0::V0;

#[derive(Clone)]
pub struct Api {
    pub v0: V0,
}

impl Api {
    pub(super) fn new(http_client: Arc<HttpClient>) -> Self {
        Self {
            v0: V0::new(Arc::clone(&http_client)),
        }
    }
}

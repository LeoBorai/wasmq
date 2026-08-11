pub mod api;

use std::sync::Arc;

use crate::client::api::Api;

#[derive(Clone)]
pub struct Client {
    pub api: Api,
}

pub(crate) struct HttpClient {
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        let http_client = Arc::new(HttpClient {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        });
        let api = Api::new(Arc::clone(&http_client));

        Self { api }
    }
}

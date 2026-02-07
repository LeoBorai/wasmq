use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;

use crate::backend::Backend;

pub struct LocalBackend {
    dir: PathBuf,
}

impl LocalBackend {
    pub async fn new() -> Result<Self> {
        todo!()
    }
}

#[async_trait]
impl Backend for LocalBackend {

}

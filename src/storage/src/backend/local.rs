use std::env::home_dir;

use anyhow::{Context, Result};
use async_trait::async_trait;
use fjall::{Database, Keyspace, KeyspaceCreateOptions};
use mate::proto::job::Job;
use tokio::fs::create_dir_all;
use uuid::Uuid;

use crate::backend::Backend;

pub struct LocalBackend {
    db: Database,
    tree: Keyspace,
}

impl LocalBackend {
    pub async fn new() -> Result<Self> {
        let home = home_dir().context("Could not find home directory")?;
        let dir = home.join(".mate").join("storage");

        if !dir.exists() {
            create_dir_all(&dir)
                .await
                .context("Could not create repository directory")?;
        }

        let db = Database::builder(dir).open()?;
        let tree = db.keyspace("jobs", KeyspaceCreateOptions::default)?;

        Ok(Self {
            db,
            tree,
        })
    }
}

#[async_trait]
impl Backend for LocalBackend {
    async fn create(&self, job: Job) -> Result<()> {
        let id = Uuid::new_v4();
        self.tree.insert(id.into_bytes(), job)?;
        Ok(())
    }
}

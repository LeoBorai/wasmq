use std::env::home_dir;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Context, Result, bail};
use async_trait::async_trait;
use bytes::Bytes;
use tokio::fs::{File, create_dir_all, read, read_dir};
use tokio::io::AsyncWriteExt;

use mate::proto::task::TaskIdentifier;

use crate::backend::Backend;

pub struct LocalBackend {
    dir: PathBuf,
}

impl LocalBackend {
    pub async fn new() -> Result<Self> {
        let home = home_dir().context("Could not find home directory")?;
        let dir = home.join(".mate").join("repository");

        if !dir.exists() {
            create_dir_all(&dir)
                .await
                .context("Could not create repository directory")?;
        }

        Ok(Self { dir })
    }
}

#[async_trait]
impl Backend for LocalBackend {
    async fn create(&self, id: &TaskIdentifier, bytes: Bytes) -> Result<()> {
        let namespace_dir = self.dir.join(&id.namespace);

        if !namespace_dir.exists() {
            create_dir_all(&namespace_dir)
                .await
                .context("Could not create namespace directory")?;
        }

        let file_path = namespace_dir.join(format!("{}@{}.wasm", id.name, id.version));

        if file_path.exists() {
            bail!("Task file already exists: {}", file_path.display());
        }

        println!("{:?}", file_path);

        let mut file = File::create_new(file_path)
            .await
            .context("Could not create task file")?;

        file.write_all(&bytes)
            .await
            .context("Could not write to task file")?;

        Ok(())
    }

    async fn find(&self, id: &TaskIdentifier) -> Result<Option<Bytes>> {
        let file_path = self
            .dir
            .join(&id.namespace)
            .join(format!("{}@{}.wasm", id.name, id.version));

        if file_path.exists() {
            let data = read(&file_path).await.context("Could not read task file")?;
            return Ok(Some(Bytes::from(data)));
        }

        Ok(None)
    }

    async fn list(&self) -> Result<Vec<TaskIdentifier>> {
        let mut tasks = Vec::new();
        let namespaces = read_dir(&self.dir)
            .await
            .context("Could not read repository directory")?;

        tokio::pin!(namespaces);

        while let Some(namespace_entry) = namespaces
            .next_entry()
            .await
            .context("Could not read namespace entry")?
        {
            let namespace_path = namespace_entry.path();
            if namespace_path.is_dir() {
                let Some(namespace) = namespace_path.file_name().and_then(|n| n.to_str()) else {
                    bail!(
                        "Failed to retrieve namespace while reading through dir. {namespace_path:?}"
                    );
                };

                let task_files = read_dir(&namespace_path)
                    .await
                    .context("Could not read namespace directory")?;

                tokio::pin!(task_files);

                while let Some(task_entry) = task_files
                    .next_entry()
                    .await
                    .context("Could not read task entry")?
                {
                    let task_path = task_entry.path();

                    if let Some(file_name) = task_path.file_name().and_then(|n| n.to_str())
                        && let Some((name_version, _)) = file_name.split_once(".wasm")
                    {
                        match TaskIdentifier::from_str(&format!("{namespace}/{}", name_version)) {
                            Ok(id) => {
                                tasks.push(id);
                            }
                            Err(e) => {
                                bail!("Warning: Skipping invalid task file '{}': {}", file_name, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(tasks)
    }
}

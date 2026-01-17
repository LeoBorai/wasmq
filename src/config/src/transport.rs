use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TransportConfig {
    UnixSocket {
        base_path: PathBuf,
    }
}

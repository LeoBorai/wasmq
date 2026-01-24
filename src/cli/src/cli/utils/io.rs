use std::fs::read_to_string;
use std::io::{Read, stdin};

use anyhow::{Result, anyhow};
use serde_json::Value;

/// Clap parser to allow parsing stdin, file or direct JSON input.
///
/// # Direct (if shell cooperates)
///
/// ```ignore
/// -x '{"hello":"world"}'
/// ```
///
/// # From file
///
/// ```ignore
/// -x @data.json
/// ```
///
/// # From stdin
///
/// ```ignore
/// echo '{"hello":"world"}' | bin -x -
/// ```
pub fn parse_json(s: &str) -> Result<Value> {
    if s == "-" {
        let mut buffer = String::new();
        stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| anyhow!("Failed to read stdin: {}", e))?;
        return serde_json::from_str(&buffer)
            .map_err(|e| anyhow!("Invalid JSON from stdin: {}", e));
    }

    if let Some(path) = s.strip_prefix('@') {
        let content =
            read_to_string(path).map_err(|e| anyhow!("Failed to read file {}: {}", path, e))?;
        return serde_json::from_str(&content)
            .map_err(|e| anyhow!("Invalid JSON in file {}: {}", path, e));
    }

    serde_json::from_str(s).map_err(|e| anyhow!("Invalid JSON: {}", e))
}

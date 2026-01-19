mod runtime;

use anyhow::Result;
use bytes::Bytes;
use serde_json::Value;

use self::runtime::wasmtime::WasmtimeRuntime;

#[derive(Clone)]
pub enum Runtime {
    Wasm(WasmtimeRuntime),
}

#[derive(Clone)]
pub struct Executor {
    runtime: Runtime,
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::Wasm(WasmtimeRuntime::new()),
        }
    }

    pub async fn run(&self, module: Bytes, input: Bytes) -> Result<Value> {
        match &self.runtime {
            Runtime::Wasm(runtime) => {
                let runtime = runtime.clone();
                tokio::spawn(async move { runtime.execute(module, input).await }).await?
            }
        }
    }
}

use anyhow::{Context, Result};
use bytes::Bytes;
use serde_json::Value;
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

const HANDLER_FUNC_FQN: &str = "handler";

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
    pub http_ctx: WasiHttpCtx,
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl WasiHttpView for ComponentRunStates {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }
}

#[derive(Clone)]
pub struct WasmtimeRuntime {}

impl WasmtimeRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(self, wasm_module: Bytes, input: Bytes) -> Result<Value> {
        let mut config = Config::new();
        config
            .wasm_component_model_async(true)
            .wasm_component_model(true)
            .wasm_component_model_async(true)
            .wasm_component_model_async_builtins(true);
        let engine = Engine::new(&config)?;
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
        wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)?;

        let json_value =
            serde_json::from_slice::<Value>(&input).context("Failed to parse input JSON")?;
        let json = serde_json::to_string(&json_value).context("Failed to serialize input JSON")?;
        let wasi = WasiCtx::builder().build();
        let state = ComponentRunStates {
            wasi_ctx: wasi,
            resource_table: ResourceTable::new(),
            http_ctx: WasiHttpCtx::new(),
        };
        let mut store = Store::new(&engine, state);
        let component = Component::from_binary(&engine, &wasm_module)?;
        let instance = linker.instantiate_async(&mut store, &component).await?;
        let func = instance
            .get_typed_func::<(String,), (Result<String, String>,)>(&mut store, HANDLER_FUNC_FQN)?;
        let (result,) = func.call_async(&mut store, (json,)).await?;

        match result {
            Ok(success) => {
                serde_json::from_str(&success).context("Failed to parse successful output JSON")
            }
            Err(failure) => Err(anyhow::anyhow!("WASM module execution failed: {}", failure)),
        }
    }
}

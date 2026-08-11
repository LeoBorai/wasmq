use wasmq_task::wasmq_handler;

#[wasmq_handler]
async fn sum(params: Vec<i64>) -> Result<i64> {
    let result: i64 = params.iter().sum();
    Ok(result)
}

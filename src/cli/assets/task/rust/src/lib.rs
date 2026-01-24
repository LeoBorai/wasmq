use mate_task::mate_handler;

/// This is your task's main function.
/// Feel free to change its name and parameters as needed.
#[mate_handler]
async fn sum(params: Vec<i64>) -> Result<i64> {
    let result: i64 = params.iter().sum();
    Ok(result)
}

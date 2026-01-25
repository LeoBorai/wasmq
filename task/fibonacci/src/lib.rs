use mate_task::mate_handler;

#[mate_handler]
async fn sum(nth: u64) -> Result<u64> {
    Ok(fibonacci_recursive(nth))
}

/// This is a simple yet inefficient recursive implementation of the Fibonacci sequence.
/// It has an exponential time complexity of O(2^n) due to the repeated calculations.
/// Used for testing purposes only.
fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

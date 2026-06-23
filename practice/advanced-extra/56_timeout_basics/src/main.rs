use std::error::Error;
use tokio::time::{Duration, error::Elapsed, sleep, timeout};

// 这里也丢个错误，方便外面确认是超时错误还是内部本身的错误
async fn simulated_slow_task(ms: usize, should_error: bool) -> Result<(), Box<dyn Error>> {
    sleep(Duration::from_millis(ms as u64)).await;
    if should_error {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "simulated error",
        )))
    } else {
        Ok(())
    }
}

async fn try_run(
    should_timeout: bool,
    should_error: bool,
) -> Result<Result<(), Box<dyn Error>>, Elapsed> {
    let task_ms = if should_timeout { 1500 } else { 500 };
    // 在这一步超时，会直接抛错，不会往下走
    let result = timeout(
        Duration::from_millis(1000),
        simulated_slow_task(task_ms, should_error),
    )
    .await?;
    // 这一步是内部有结果了，可能失败可能成功
    Ok(result)
}

fn matcher(result: Result<Result<(), Box<dyn Error>>, Elapsed>) {
    match result {
        Ok(Ok(_)) => println!("Task completed successfully."),
        Ok(Err(e)) => println!("Task completed with error: {e}"),
        Err(_) => println!("Task timed out"),
    }
}

#[tokio::main]
async fn main() {
    println!("This should be timeout");
    let result = try_run(true, false).await;
    matcher(result);

    println!("This should be success");
    let result = try_run(false, false).await;
    matcher(result);

    println!("This should catch inner error");
    let result = try_run(false, true).await;
    matcher(result);
}

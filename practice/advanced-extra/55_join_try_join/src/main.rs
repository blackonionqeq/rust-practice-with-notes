use std::cmp;
use std::time::Instant;

use thiserror::Error;
use tokio::time::{self, Duration};

#[derive(Debug, Error)]
enum AppError {
    #[error("task {0} failed")]
    TaskFailed(&'static str),
}

// 供 join! 使用：两个任务都不会失败，直接返回数值。
async fn sleep_ms(timeout_ms: usize) -> usize {
    time::sleep(Duration::from_millis(timeout_ms as u64)).await;
    timeout_ms
}

async fn sleep_secs(timeout_secs: usize) -> usize {
    // as 来类型转换
    time::sleep(Duration::from_secs(timeout_secs as u64)).await;
    timeout_secs
}

// 供 try_join! 使用：两个任务使用相同的 AppError 错误类型。
async fn fallible_sleep_ms(timeout_ms: usize, should_fail: bool) -> Result<usize, AppError> {
    time::sleep(Duration::from_millis(timeout_ms as u64)).await;

    if should_fail {
        Err(AppError::TaskFailed("millisecond task"))
    } else {
        Ok(timeout_ms)
    }
}

async fn fallible_sleep_secs(timeout_secs: usize, should_fail: bool) -> Result<usize, AppError> {
    time::sleep(Duration::from_secs(timeout_secs as u64)).await;

    if should_fail {
        Err(AppError::TaskFailed("second task"))
    } else {
        Ok(timeout_secs)
    }
}

// join! 会等待所有 Future 完成，并直接返回结果元组。
async fn join_example() -> usize {
    let task_a_ms = 2_000;
    let task_b_secs = 1;
    let expected_ms = cmp::max(task_a_ms, task_b_secs * 1_000);

    println!("join!: expected time is about {expected_ms} ms");
    let start = Instant::now();

    let (result_a_ms, result_b_secs) = tokio::join!(sleep_ms(task_a_ms), sleep_secs(task_b_secs));

    let total_ms = result_a_ms + result_b_secs * 1_000;
    println!(
        "join!: took {} ms, combined total is {total_ms} ms",
        start.elapsed().as_millis()
    );

    total_ms
}

// try_join! 在全部成功时返回元组，任一任务失败时立即返回 Err。
async fn try_join_example(should_fail: bool) -> Result<usize, AppError> {
    let task_a_ms = 2_000;
    let task_b_secs = 1;
    let expected_ms = if should_fail {
        task_a_ms.min(task_b_secs * 1_000)
    } else {
        task_a_ms.max(task_b_secs * 1_000)
    };

    println!("try_join!: expected time is about {expected_ms} ms");
    let start = Instant::now();

    let result = tokio::try_join!(
        fallible_sleep_ms(task_a_ms, false),
        fallible_sleep_secs(task_b_secs, should_fail),
    );

    println!("try_join!: took {} ms", start.elapsed().as_millis());

    // ? 会保留 try_join! 返回的原始 AppError，并在失败时立即返回。
    let (result_a_ms, result_b_secs) = result?;

    let total_ms = result_a_ms + result_b_secs * 1_000;
    println!("try_join!: combined total is {total_ms} ms");

    Ok(total_ms)
}

#[tokio::main]
async fn main() {
    join_example().await;

    match try_join_example(false).await {
        Ok(total_ms) => println!("try_join! success: {total_ms} ms"),
        Err(error) => println!("try_join! error: {error}"),
    }

    match try_join_example(true).await {
        Ok(total_ms) => println!("try_join! success: {total_ms} ms"),
        Err(error) => println!("try_join! failed fast: {error}"),
    }
}

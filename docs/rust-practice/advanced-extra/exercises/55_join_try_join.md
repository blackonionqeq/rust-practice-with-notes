# 55 Join and Try Join

## Goal

Practice waiting for multiple async operations.

## Project

Create project:

```bash
cargo new practice/advanced-extra/55_join_try_join --name join_try_join
```

## Requirements

- Create two or three async functions that return `Result<usize, AppError>`.
- Use `tokio::try_join!` to run them concurrently and fail fast on error.
- Also demonstrate `tokio::join!` for infallible async functions.
- Print combined totals on success.

## Constraints

- Do not await each operation sequentially when they are independent.
- Do not use `unwrap` or `expect`.
- Keep the custom error type small.
- Do not hand-write futures.

## Review Focus

- Concurrent waiting vs sequential waiting.
- Correct error behavior with `try_join!`.
- Readable async control flow.

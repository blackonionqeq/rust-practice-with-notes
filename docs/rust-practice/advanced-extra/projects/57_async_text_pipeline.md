# 57 Async Text Pipeline

## Goal

Build an async text pipeline using application-level async/await, Tokio file IO, concurrent waits, and timeouts.

This is an advanced-extra integration project.

## Project

Create project:

```bash
cargo new practice/advanced-extra/57_async_text_pipeline --name async_text_pipeline
```


## Requirements

- Use `tokio` as the async runtime.
- Read multiple files asynchronously.
- Run independent file reads concurrently with `try_join!` or spawned tasks.
- Apply a timeout to each slow operation.
- Aggregate successful results and report failures clearly.

## Constraints

- Do not hand-write `Future` implementations.
- Do not use `Pin` directly.
- Do not block the async runtime with synchronous sleeps or long CPU work.
- Keep pure text counting logic separate from async IO.

## Review Focus

- Practical project boundaries.
- Error messages and error propagation.
- Testability of core logic.
- Ownership choices around IO, concurrency, or async boundaries.
- Formatting with `cargo fmt`.

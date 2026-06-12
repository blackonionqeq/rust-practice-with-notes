# 56 Timeout Basics

## Goal

Practice applying timeouts to async operations.

## Project

Create project:

```bash
cargo new practice/advanced-extra/56_timeout_basics --name timeout_basics
```

## Requirements

- Use `tokio::time::timeout` around a simulated slow operation.
- Return a clear timeout error when the operation takes too long.
- Demonstrate one successful operation and one timed-out operation.
- Print user-friendly results.

## Constraints

- Use `tokio::time::{sleep, timeout, Duration}`.
- Do not block the async runtime with `std::thread::sleep`.
- Do not use `unwrap` or `expect`.
- Keep cancellation behavior conceptual; do not implement custom cancellation.

## Review Focus

- Correct timeout wrapping.
- Avoiding blocking calls in async code.
- Clear distinction between operation error and timeout error.

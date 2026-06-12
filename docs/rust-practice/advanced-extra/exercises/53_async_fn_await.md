# 53 Async Fn and Await

## Goal

Practice the surface syntax of `async fn` and `.await`.

## Project

Create project:

```bash
cargo new practice/advanced-extra/53_async_fn_await --name async_fn_await
```

## Requirements

- Add `tokio` with `macros`, `rt-multi-thread`, and `time` features.
- Create `async fn fetch_label(id: u32) -> String` that waits briefly then returns a label.
- Use `#[tokio::main]` on `main`.
- Call the async function with `.await` and print the result.

## Constraints

- Do not hand-write a `Future`.
- Do not discuss or use `Pin` in code.
- Use `tokio::time::sleep` for the simulated async delay.
- Keep this exercise focused on application-level async syntax.

## Review Focus

- Correct async function structure.
- Correct runtime setup.
- Understanding that async work runs when awaited or spawned.

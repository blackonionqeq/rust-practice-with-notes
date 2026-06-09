# 31 Arc Mutex Threads

## Goal

Practice `Arc<T>`, `Mutex<T>`, and sharing mutable state across threads.

## Project

Create project:

```bash
cargo new practice/advanced/31_arc_mutex_threads --name arc_mutex_threads
```

## Required Function

```rust
fn run_counter(thread_count: usize, increments_per_thread: usize) -> i32
```

Create a shared counter initialized to `0`.

Spawn `thread_count` threads. Each thread increments the counter `increments_per_thread` times.

Return the final counter value.

## Main Program

In `main`:

1. Call `run_counter(4, 1000)`.
2. Print `count: {count}`.

Expected output:

```text
count: 4000
```

## Constraints

- Use `std::sync::Arc`.
- Use `std::sync::Mutex`.
- Use `std::thread::spawn`.
- Use `Arc::clone` for each thread.
- Join every thread.
- Do not use `static mut`.
- Do not use `unsafe`.
- Do not use external crates.

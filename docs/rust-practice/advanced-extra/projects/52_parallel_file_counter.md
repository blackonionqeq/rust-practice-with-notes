# 52 Parallel File Counter

## Goal

Build a multi-threaded file counter that compares message passing with shared-state approaches.

This is an advanced-extra integration project.

## Project

Create project:

```bash
cargo new practice/advanced-extra/52_parallel_file_counter --name parallel_file_counter
```


## Requirements

- Accept multiple input files.
- Spawn worker threads to process files concurrently.
- Send per-file results to the main thread with `mpsc`.
- Aggregate total lines, words, and bytes in main.
- Print one error per failed file without stopping all workers.

## Constraints

- Prefer message passing over `Arc<Mutex<HashMap<_>>>` for the main implementation.
- Join all worker threads.
- Do not use `unwrap` or `expect` for thread/channel errors.
- Keep lock usage minimal if you add an optional shared-state comparison.

## Review Focus

- Practical project boundaries.
- Error messages and error propagation.
- Testability of core logic.
- Ownership choices around IO, concurrency, or async boundaries.
- Formatting with `cargo fmt`.

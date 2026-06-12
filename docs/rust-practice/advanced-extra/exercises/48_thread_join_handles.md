# 48 Thread Join Handles

## Goal

Practice spawning threads and collecting their return values.

## Project

Create project:

```bash
cargo new practice/advanced-extra/48_thread_join_handles --name thread_join_handles
```

## Requirements

- Create a list of numbers.
- Spawn at least three threads.
- Each thread should compute a partial sum.
- Join all threads and combine the results.
- Return or print the final sum.

## Constraints

- Move owned data into threads safely.
- Handle thread panics from `join` without `unwrap`.
- Do not use shared mutable state for this exercise.
- Keep each thread workload independent.

## Review Focus

- Ownership transfer into threads.
- Correct use of `JoinHandle`.
- Avoiding unnecessary shared state.

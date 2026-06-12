# 49 MPSC Worker Results

## Goal

Practice message passing with `std::sync::mpsc`.

## Project

Create project:

```bash
cargo new practice/advanced-extra/49_mpsc_worker_results --name mpsc_worker_results
```

## Requirements

- Create several worker threads.
- Each worker computes a result from owned input data.
- Send each result back to the main thread through a channel.
- Drop the original sender so the receiver loop can finish.
- Aggregate and print the results in main.

## Constraints

- Use `std::sync::mpsc::channel`.
- Clone the sender for workers.
- Do not share a mutable `Vec` behind `Mutex`.
- Do not use `unwrap` or `expect` for send/receive failures.

## Review Focus

- Correct channel lifecycle.
- Message passing instead of shared mutation.
- Handling worker and channel errors clearly.

# Rust Practice Advanced Extra

This directory tracks practical Rust project skills that are useful after the core advanced section.

The current `advanced` section focuses on core language and standard-library mechanisms. `advanced-extra` focuses on real project skills: CLI design, Cargo ecosystem, serde, practical errors, integration tests, workspaces, concurrency patterns, and application-level async.

## Relationship to Other Sections

- Basic: language fundamentals.
- Advanced: core intermediate Rust mechanisms.
- Advanced Extra: practical Rust project skills and ecosystem usage.
- Expert: boundary and lower-level topics such as macros, unsafe, FFI, `Pin`, DST, and self-reference.

## Current Step

Start after advanced project `32_task_tree`.

First exercise:

- `docs/rust-practice/advanced-extra/exercises/33_cli_args_pathbuf.md`

## Stage X1: Real CLI and IO

Goal: move from fixed sample files to real command line programs that accept paths, config, and large inputs.

### Topics

1. CLI arguments
2. `Path` / `PathBuf`
3. Environment variables
4. Streaming IO with `BufReader`
5. Basic typed CLI with `clap`

### Exercises

- `33_cli_args_pathbuf`: `docs/rust-practice/advanced-extra/exercises/33_cli_args_pathbuf.md`
- `34_streaming_word_count`: `docs/rust-practice/advanced-extra/exercises/34_streaming_word_count.md`
- `35_env_config`: `docs/rust-practice/advanced-extra/exercises/35_env_config.md`
- `36_clap_text_stats`: `docs/rust-practice/advanced-extra/exercises/36_clap_text_stats.md`

### Supplement Notes

- `33_cli_args_pathbuf`: `docs/rust-practice/advanced-extra/notes/33_cli_args_pathbuf.md`
- `34_streaming_word_count`: `docs/rust-practice/advanced-extra/notes/34_streaming_word_count.md`
- `35_env_config`: `docs/rust-practice/advanced-extra/notes/35_env_config.md`
- `36_clap_text_stats`: `docs/rust-practice/advanced-extra/notes/36_clap_text_stats.md`

### Project

- `37_real_text_stats_cli`: `docs/rust-practice/advanced-extra/projects/37_real_text_stats_cli.md`

## Stage X2: Crate Ecosystem and Practical Errors

Goal: learn a small, realistic set of ecosystem crates and the error-handling style common in Rust applications.

### Topics

1. `thiserror`
2. `anyhow`
3. `serde`
4. `serde_json`
5. `dev-dependencies` and `tempfile`

### Exercises

- `38_thiserror_app_error`: `docs/rust-practice/advanced-extra/exercises/38_thiserror_app_error.md`
- `39_anyhow_context`: `docs/rust-practice/advanced-extra/exercises/39_anyhow_context.md`
- `40_serde_json_config`: `docs/rust-practice/advanced-extra/exercises/40_serde_json_config.md`
- `41_dev_dependencies_tempfile`: `docs/rust-practice/advanced-extra/exercises/41_dev_dependencies_tempfile.md`

### Supplement Notes

- `38_thiserror_app_error`: `docs/rust-practice/advanced-extra/notes/38_thiserror_app_error.md`
- `39_anyhow_context`: `docs/rust-practice/advanced-extra/notes/39_anyhow_context.md`
- `40_serde_json_config`: `docs/rust-practice/advanced-extra/notes/40_serde_json_config.md`
- `41_dev_dependencies_tempfile`: `docs/rust-practice/advanced-extra/notes/41_dev_dependencies_tempfile.md`

### Project

- `42_configurable_text_stats`: `docs/rust-practice/advanced-extra/projects/42_configurable_text_stats.md`

## Stage X3: Testing and Project Engineering

Goal: organize Rust code like a small real project, with public APIs, integration tests, docs, workspaces, and optional features.

### Topics

1. Integration tests
2. Doc tests
3. Cargo workspace
4. Feature flags

### Exercises

- `43_integration_tests`: `docs/rust-practice/advanced-extra/exercises/43_integration_tests.md`
- `44_doc_tests`: `docs/rust-practice/advanced-extra/exercises/44_doc_tests.md`
- `45_workspace_basics`: `docs/rust-practice/advanced-extra/exercises/45_workspace_basics.md`
- `46_feature_flags`: `docs/rust-practice/advanced-extra/exercises/46_feature_flags.md`

### Supplement Notes

- `43_integration_tests`: `docs/rust-practice/advanced-extra/notes/43_integration_tests.md`
- `44_doc_tests`: `docs/rust-practice/advanced-extra/notes/44_doc_tests.md`
- `45_workspace_basics`: `docs/rust-practice/advanced-extra/notes/45_workspace_basics.md`
- `46_feature_flags`: `docs/rust-practice/advanced-extra/notes/46_feature_flags.md`

### Project

- `47_text_tools_workspace`: `docs/rust-practice/advanced-extra/projects/47_text_tools_workspace.md`

## Stage X4: Concurrency Beyond Mutex

Goal: expand from `Arc<Mutex<_>>` to practical thread ownership, message passing, read/write locks, and lock-scope discipline.

Related guide:

- `concurrency_choice`: `docs/rust-practice/advanced-extra/guides/concurrency_choice.md`

### Topics

1. `thread::spawn`
2. `JoinHandle`
3. `std::sync::mpsc`
4. `RwLock`
5. Lock scope and deadlock avoidance

### Exercises

- `48_thread_join_handles`: `docs/rust-practice/advanced-extra/exercises/48_thread_join_handles.md`
- `49_mpsc_worker_results`: `docs/rust-practice/advanced-extra/exercises/49_mpsc_worker_results.md`
- `50_rwlock_read_mostly`: `docs/rust-practice/advanced-extra/exercises/50_rwlock_read_mostly.md`
- `51_lock_scope_deadlock`: `docs/rust-practice/advanced-extra/exercises/51_lock_scope_deadlock.md`

### Supplement Notes

- `48_thread_join_handles`: `docs/rust-practice/advanced-extra/notes/48_thread_join_handles.md`
- `49_mpsc_worker_results`: `docs/rust-practice/advanced-extra/notes/49_mpsc_worker_results.md`
- `50_rwlock_read_mostly`: `docs/rust-practice/advanced-extra/notes/50_rwlock_read_mostly.md`
- `51_lock_scope_deadlock`: `docs/rust-practice/advanced-extra/notes/51_lock_scope_deadlock.md`

### Project

- `52_parallel_file_counter`: `docs/rust-practice/advanced-extra/projects/52_parallel_file_counter.md`

## Stage X5: Async Basics

Goal: learn application-level async Rust without diving into `Pin`, hand-written futures, or executor internals.

Related guides:

- `concurrency_choice`: `docs/rust-practice/advanced-extra/guides/concurrency_choice.md`
- `async_boundary`: `docs/rust-practice/advanced-extra/guides/async_boundary.md`

### Topics

1. `async fn`
2. `.await`
3. Tokio runtime basics
4. Async file IO
5. `join!` / `try_join!`
6. `timeout`

### Exercises

- `53_async_fn_await`: `docs/rust-practice/advanced-extra/exercises/53_async_fn_await.md`
- `54_tokio_file_read`: `docs/rust-practice/advanced-extra/exercises/54_tokio_file_read.md`
- `55_join_try_join`: `docs/rust-practice/advanced-extra/exercises/55_join_try_join.md`
- `56_timeout_basics`: `docs/rust-practice/advanced-extra/exercises/56_timeout_basics.md`

### Supplement Notes

- `53_async_fn_await`: `docs/rust-practice/advanced-extra/notes/53_async_fn_await.md`
- `54_tokio_file_read`: `docs/rust-practice/advanced-extra/notes/54_tokio_file_read.md`
- `55_join_try_join`: `docs/rust-practice/advanced-extra/notes/55_join_try_join.md`
- `56_timeout_basics`: `docs/rust-practice/advanced-extra/notes/56_timeout_basics.md`

### Project

- `57_async_text_pipeline`: `docs/rust-practice/advanced-extra/projects/57_async_text_pipeline.md`

## Next Section

Advanced Extra is followed by Expert:

- `docs/rust-practice/expert/README.md`

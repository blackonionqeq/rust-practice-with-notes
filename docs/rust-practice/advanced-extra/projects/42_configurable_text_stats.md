# 42 Configurable Text Stats

## Goal

Build a configurable text statistics tool using `serde`, `thiserror`, `anyhow`, and test-only dependencies.

This is an advanced-extra integration project.

## Project

Create project:

```bash
cargo new practice/advanced-extra/42_configurable_text_stats --name configurable_text_stats
```


## Requirements

- Read a JSON config file path from CLI arguments.
- Deserialize config with `serde_json`.
- Use `thiserror` for library errors.
- Use `anyhow` with context in the binary entry point.
- Use `tempfile` in tests for temporary input files.

## Constraints

- Keep library functions independent from CLI parsing.
- Preserve source errors where useful.
- Do not use `unwrap` or `expect` in production code.
- Put test-only crates under `[dev-dependencies]`.

## Review Focus

- Practical project boundaries.
- Error messages and error propagation.
- Testability of core logic.
- Ownership choices around IO, concurrency, or async boundaries.
- Formatting with `cargo fmt`.

# 40 Serde JSON Config

## Goal

Practice parsing a JSON configuration file into typed Rust structs.

## Project

Create project:

```bash
cargo new practice/advanced-extra/40_serde_json_config --name serde_json_config
```

## Requirements

- Add `serde` with `derive` and `serde_json`.
- Define `struct Config { input: PathBuf, min_count: Option<usize>, ignore_case: bool }`.
- Read a config path from CLI arguments.
- Parse JSON into `Config`.
- Apply default `min_count = 2` when the field is missing or null.

## Constraints

- Use `#[derive(serde::Deserialize, Debug)]`.
- Use `PathBuf` for paths.
- Do not use `unwrap` or `expect`.
- Return clear errors for file read and JSON parse failures.

## Review Focus

- Typed config design.
- Appropriate use of `Option` and defaults.
- Deserialization error handling.

# 54 Tokio File Read

## Goal

Practice async file reading with `tokio::fs`.

## Project

Create project:

```bash
cargo new practice/advanced-extra/54_tokio_file_read --name tokio_file_read
```

## Requirements

- Add `tokio` with `macros`, `rt-multi-thread`, and `fs` features.
- Read a path from CLI arguments.
- Define `async fn read_text(path: &Path) -> std::io::Result<String>`.
- Use `tokio::fs::read_to_string`.
- Print the number of lines and words.

## Constraints

- Use async file API, not `std::fs::read_to_string`.
- Do not use `unwrap` or `expect`.
- Keep parsing/counting logic synchronous unless it performs IO.
- Use `Path` / `PathBuf` for paths.

## Review Focus

- Async IO boundary.
- Separation of async IO and pure computation.
- Error propagation from async functions.

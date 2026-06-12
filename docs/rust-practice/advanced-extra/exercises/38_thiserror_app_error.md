# 38 Thiserror App Error

## Goal

Practice defining practical library errors with the `thiserror` crate.

## Project

Create project:

```bash
cargo new practice/advanced-extra/38_thiserror_app_error --name thiserror_app_error
```

## Requirements

- Add `thiserror` as a dependency.
- Define `#[derive(Debug, thiserror::Error)] enum AppError`.
- Include variants for IO, parse integer, and invalid input.
- Use `#[from]` where appropriate.
- Write a function that reads a file and parses one number per non-empty line.

## Constraints

- Do not hand-write `Display` for this exercise.
- Preserve source errors for IO and parse failures.
- Do not use `unwrap` or `expect`.
- Return `Result<Vec<i64>, AppError>` from the parser function.

## Review Focus

- Accurate error variants.
- Whether original source errors are preserved.
- Library-style concrete error type design.

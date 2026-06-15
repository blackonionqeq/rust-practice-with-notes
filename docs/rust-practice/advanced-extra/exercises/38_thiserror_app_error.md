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
- Write a function that streams a file line by line and parses one number per non-empty line.
- Use `std::fs::File`, `std::io::BufReader`, and `std::io::BufRead::lines`.

## Constraints

- Do not hand-write `Display` for this exercise.
- Preserve source errors for IO and parse failures.
- Do not use `unwrap` or `expect`.
- Return `Result<Vec<i64>, AppError>` from the parser function.
- Do not use `std::fs::read_to_string` in the final implementation.
- It is acceptable to keep a commented reference version using `read_to_string`, but mark it as the non-streaming version.

## Review Focus

- Accurate error variants.
- Whether original source errors are preserved.
- Whether IO errors from both `File::open` and `lines()` are propagated.
- Library-style concrete error type design.

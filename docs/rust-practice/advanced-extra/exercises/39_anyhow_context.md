# 39 Anyhow Context

## Goal

Practice binary-level error handling with `anyhow` and contextual messages.

## Project

Create project:

```bash
cargo new practice/advanced-extra/39_anyhow_context --name anyhow_context
```

## Requirements

- Add `anyhow` as a dependency.
- Define `fn run() -> anyhow::Result<()>`.
- Read a path from CLI arguments.
- Read the file and parse integers from lines.
- Use `.context(...)` or `.with_context(...)` to explain each fallible step.
- Make `main` print the error chain with `eprintln!`.

## Constraints

- Use `anyhow` only at the binary/application boundary.
- Do not define a custom error enum in this exercise.
- Do not use `unwrap` or `expect`.
- Add useful context that includes the path or line number when possible.

## Review Focus

- Quality of error context.
- Clean `run() -> anyhow::Result<()>` structure.
- Difference between application errors and library errors.

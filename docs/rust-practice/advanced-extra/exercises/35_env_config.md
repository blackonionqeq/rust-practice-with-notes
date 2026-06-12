# 35 Environment Config

## Goal

Practice reading environment variables and combining them with default configuration.

## Project

Create project:

```bash
cargo new practice/advanced-extra/35_env_config --name env_config
```

## Requirements

- Define `struct Config { input: std::path::PathBuf, min_count: usize }`.
- Read input path from CLI argument first.
- If CLI input is missing, read `TEXT_INPUT` from environment.
- Read optional `MIN_COUNT` from environment.
- Default `min_count` to `2` when not set.
- Return a custom error when neither CLI nor `TEXT_INPUT` provides input.

## Constraints

- Use `std::env::args` and `std::env::var`.
- Parse `MIN_COUNT` safely with `parse::<usize>()`.
- Do not use `unwrap` or `expect`.
- Keep config parsing separate from file processing.

## Review Focus

- Priority between CLI args, environment variables, and defaults.
- Error handling around missing and invalid config.
- Separation between configuration and business logic.

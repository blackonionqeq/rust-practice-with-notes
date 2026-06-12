# 36 Clap Text Stats

## Goal

Practice using the `clap` crate for a small typed CLI.

## Project

Create project:

```bash
cargo new practice/advanced-extra/36_clap_text_stats --name clap_text_stats
```

## Requirements

- Add `clap` with the `derive` feature.
- Define `#[derive(clap::Parser)] struct Args`.
- Support `--input <PATH>`.
- Support `--min-count <N>` with default `2`.
- Support `--ignore-case` as a bool flag.
- Print parsed arguments and a simple word count result.

## Constraints

- Use `PathBuf` for the input argument.
- Do not manually parse `std::env::args` in this exercise.
- Do not use `unwrap` or `expect` in application logic.
- Keep external crate usage limited to `clap`.

## Review Focus

- Correct `clap` derive usage.
- Typed CLI fields and sensible defaults.
- Clear user-facing command line behavior.

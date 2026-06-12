# 37 Real Text Stats CLI

## Goal

Build a more realistic text statistics CLI using `PathBuf`, CLI arguments, environment fallback, and streaming IO.

This is an advanced-extra integration project.

## Project

Create project:

```bash
cargo new practice/advanced-extra/37_real_text_stats_cli --name real_text_stats_cli
```


## Requirements

- Support `--input <PATH>` or a positional input path.
- Support `--min-count <N>` with default `2`.
- Support an `--ignore-case` flag or equivalent config field.
- Use `BufReader` to process input line by line.
- Print total lines, total words, unique words, and repeated words.

## Constraints

- Do not load the whole file with `read_to_string`.
- Use `PathBuf` for user-provided paths.
- Do not use `unwrap` or `expect`.
- External crates are optional for this project; if using `clap`, keep parsing code separate from counting logic.

## Review Focus

- Practical project boundaries.
- Error messages and error propagation.
- Testability of core logic.
- Ownership choices around IO, concurrency, or async boundaries.
- Formatting with `cargo fmt`.

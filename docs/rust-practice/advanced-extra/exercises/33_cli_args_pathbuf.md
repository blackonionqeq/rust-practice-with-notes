# 33 CLI Args and PathBuf

## Goal

Practice reading command line arguments and using `PathBuf` for file paths.

## Project

Create project:

```bash
cargo new practice/advanced-extra/33_cli_args_pathbuf --name cli_args_pathbuf
```

## Requirements

- Read the input path from the first command line argument.
- Store the path as `std::path::PathBuf`.
- Print `usage: cli_args_pathbuf <input-file>` when the argument is missing.
- Read the file with `std::fs::read_to_string`.
- Print the file path with `display()` and the byte length of the content.

## Constraints

- Do not hard-code `sample.txt`.
- Do not use `unwrap` or `expect`.
- Use `PathBuf`, not plain `String`, to represent file paths.
- Keep all code in `src/main.rs`.

## Review Focus

- Correct CLI argument handling.
- Use of `PathBuf` / `Path` instead of stringly typed paths.
- Clear error messages for missing argument and file read failure.

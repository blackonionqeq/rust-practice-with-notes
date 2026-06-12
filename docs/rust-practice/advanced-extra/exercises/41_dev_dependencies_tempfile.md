# 41 Dev Dependencies and Tempfile

## Goal

Practice using `dev-dependencies` and temporary files in tests.

## Project

Create project:

```bash
cargo new practice/advanced-extra/41_dev_dependencies_tempfile --name dev_dependencies_tempfile
```

## Requirements

- Add `tempfile` under `[dev-dependencies]`.
- Write a library function that counts lines in a file.
- Add tests that create temporary files with sample content.
- Verify empty, one-line, and multi-line files.

## Constraints

- Do not create test files in the repository root.
- Use `tempfile::NamedTempFile` or `tempfile::tempdir`.
- Do not use `unwrap`; in tests return `Result<(), Box<dyn std::error::Error>>` where helpful.
- Keep production dependencies separate from dev dependencies.

## Review Focus

- Correct use of `dev-dependencies`.
- Tests that do not leave files behind.
- Clean test error handling.

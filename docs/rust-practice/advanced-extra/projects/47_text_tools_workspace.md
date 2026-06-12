# 47 Text Tools Workspace

## Goal

Refactor text processing into a small Cargo workspace with a core library and CLI binary.

This is an advanced-extra integration project.

## Project

Create project:

```bash
cargo new practice/advanced-extra/47_text_tools_workspace --name text_tools_workspace
```

For workspace projects, create the directory first and initialize member crates manually as needed.

## Requirements

- Create a workspace root with `[workspace]`.
- Create `crates/text_core` as a library crate.
- Create `crates/text_cli` as a binary crate.
- Add integration tests for the public `text_core` API.
- Add a `json` feature for optional JSON output.

## Constraints

- Do not duplicate text processing logic in the CLI crate.
- Make feature-gated code compile both with and without the feature.
- Include at least one doc test for a public function.
- Run tests from the workspace root.

## Review Focus

- Practical project boundaries.
- Error messages and error propagation.
- Testability of core logic.
- Ownership choices around IO, concurrency, or async boundaries.
- Formatting with `cargo fmt`.

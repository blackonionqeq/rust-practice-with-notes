# 45 Workspace Basics

## Goal

Practice organizing multiple crates in one Cargo workspace.

## Project

Create project:

```bash
cargo new practice/advanced-extra/45_workspace_basics --name workspace_basics
```

## Requirements

- Create a workspace project directory.
- Create two member crates: `text_core` library and `text_cli` binary.
- `text_cli` should depend on `text_core` by path.
- Move text counting logic into `text_core`.
- Keep CLI parsing and printing in `text_cli`.

## Constraints

- Use a workspace-level `Cargo.toml` with `[workspace]`.
- Do not duplicate business logic between crates.
- Run tests from the workspace root.
- Keep package names valid and non-numeric.

## Review Focus

- Workspace layout.
- Clear crate responsibilities.
- Path dependency configuration.

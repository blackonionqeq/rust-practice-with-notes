# 45 Workspace Basics

## Goal

Practice organizing multiple crates in one Cargo workspace.

## Project

Create the workspace root directory:

```bash
mkdir -p practice/advanced-extra/45_workspace_basics
cd practice/advanced-extra/45_workspace_basics
```

PowerShell equivalent:

```powershell
New-Item -ItemType Directory -Force practice/advanced-extra/45_workspace_basics
Set-Location practice/advanced-extra/45_workspace_basics
```

Create member crates with Cargo:

```bash
cargo new crates/text_core --lib --name text_core
cargo new crates/text_cli --bin --name text_cli
```

Create the workspace root `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/text_core",
    "crates/text_cli",
]
resolver = "2"
```

Make `text_cli` depend on `text_core` in `crates/text_cli/Cargo.toml`:

```toml
[dependencies]
text_core = { path = "../text_core" }
```

Expected layout:

```text
45_workspace_basics/
├── Cargo.toml
└── crates/
    ├── text_core/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── text_cli/
        ├── Cargo.toml
        └── src/main.rs
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

## Verification

From `practice/advanced-extra/45_workspace_basics`:

```bash
cargo fmt --check
cargo test
cargo run -p text_cli -- "hello rust rust"
```

## Review Focus

- Workspace layout.
- Clear crate responsibilities.
- Path dependency configuration.

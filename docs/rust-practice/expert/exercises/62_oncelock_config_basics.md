# 62 OnceLock Config Basics

## Goal

Build a tiny, safe global read-only config with one-time initialization.

## Project

Create project:

```bash
cargo new practice/expert/62_oncelock_config_basics --name oncelock_config_basics
```

## Requirements

Use:

- `std::sync::OnceLock`
- `std::collections::HashMap`

Define:

```rust
static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
```

```rust
fn init_config() -> Result<(), &'static str>
```

Initialize the map once. Return `Err("config already initialized")` when called again.

```rust
fn get_config(key: &str) -> Option<&'static str>
```

Read from initialized config.

In `main`:

1. Call `init_config()` and handle result.
2. Read and print at least two keys.
3. Call `init_config()` again and print the second-call error.

## Learning Scope

Already learned tools:

- `HashMap`
- basic error handling with `Result` / `Option`
- `static` basics from docs

New knowledge points in this exercise:

- `OnceLock` one-time initialization pattern
- safe global read-only state
- explicit handling of repeated initialization

Out of scope (do not use yet):

- `static mut`
- atomics
- multi-threaded mutation patterns
- `unsafe`

## Constraints

- Keep code in `src/main.rs`.
- Do not use `unsafe`.
- Do not use extra crates.
- Keep global state read-only after init.

## Review Focus

- Correct one-time initialization behavior.
- Safe global access without `static mut`.

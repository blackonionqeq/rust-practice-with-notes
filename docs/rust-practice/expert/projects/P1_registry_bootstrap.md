# P1 Registry Bootstrap

## Goal

Integrate E1 concepts into one small program: static references, intentional leaks, DST-friendly read API, and safe one-time initialization.

This is an expert integration project after exercise 62.

## Project

Create project:

```bash
cargo new practice/expert/P1_registry_bootstrap --name registry_bootstrap
```

## Requirements

Build a read-only string registry with the following API:

```rust
fn init_registry(entries: &[(&'static str, &'static str)]) -> Result<(), &'static str>
fn get(key: &str) -> Option<&'static str>
```

- `init_registry` accepts a slice of key-value pairs where both key and value are `&'static str`.
- `get` looks up a key in the initialized registry.
- Use `OnceLock<HashMap<&'static str, &'static str>>` for the backing storage.
- Calling `init_registry` a second time should return `Err("already initialized")`.

Additionally:

- Add a helper function `register_owned(key: &'static str, value: String) -> Result<(), &'static str>`.
  - This function should use `Box::leak` to convert the owned `String` into a `&'static str` before storing.
  - Document why `Box::leak` is acceptable here (one-time setup, process lifetime).
- Expose a `list_keys() -> &'static [&'static str]` or similar that returns the known keys via a `Vec` collected at lookup time.

In `main`:

1. Initialize the registry with two static string pairs.
2. Register one owned string value using `register_owned`.
3. Look up and print three keys (two present, one absent).
4. Attempt to call `init_registry` a second time and print the error.

## Constraints

- Keep all code in `src/main.rs`.
- Do not use `unsafe` directly (rely on `Box::leak` from safe Rust).
- Do not use extra crates.
- Write at least one test that verifies a successful lookup.

## Review Focus

- Correct use of `OnceLock` for initialization boundary.
- `Box::leak` is justified by documented invariant (process-lifetime value).
- API uses `&str` input types, not `String` (DST-friendly boundary).
- Second init returns error, not silently ignored.
- Test covers the core lookup path.

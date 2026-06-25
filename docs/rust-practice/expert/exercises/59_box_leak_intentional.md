# 59 Box Leak (Intentional)

## Goal

Understand intentional leaks with `Box::leak` and when they are acceptable.

## Project

Create project:

```bash
cargo new practice/expert/59_box_leak_intentional --name box_leak_intentional
```

## Requirements

Define and use:

```rust
fn make_static_label(input: String) -> &'static str
```

Implementation requirements:

1. Convert `input` into boxed string data.
2. Use `Box::leak` to return `&'static str`.

In `main`:

1. Create two owned strings.
2. Convert each using `make_static_label`.
3. Print both leaked labels.
4. Add a short note comment: this memory is intentionally never freed.

## Learning Scope

Already learned tools:

- ownership moves
- references and lifetimes
- `String` / `&str`

New knowledge points in this exercise:

- `String` to `Box<str>` conversion
- `Box::leak` as an intentional one-way conversion
- leak as a deliberate memory-management trade-off

Out of scope (do not use yet):

- `OnceLock` or global registries
- atomics
- `unsafe`

## Constraints

- Keep code in `src/main.rs`.
- Do not use `unsafe`.
- Do not use extra crates.
- Keep the example small (single helper function + `main`).

## Review Focus

- Correct use of `Box::leak`.
- Clear explanation that leaking is a trade-off, not a default pattern.

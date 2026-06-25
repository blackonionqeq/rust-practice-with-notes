# 58 Static Lifetime Basics

## Goal

Distinguish `&'static T` from `T: 'static` with minimal examples.

## Project

Create project:

```bash
cargo new practice/expert/58_static_lifetime_basics --name static_lifetime_basics
```

## Requirements

Define and use:

```rust
fn takes_static_ref(s: &'static str) -> usize
```

Return `s.len()`.

```rust
fn takes_static_bound<T: 'static>(value: T) -> T
```

Return `value` directly.

In `main`:

1. Pass a string literal to `takes_static_ref` and print the length.
2. Pass an owned `String` to `takes_static_bound` and print it.
3. Add a short comment (2-3 lines) explaining why `&local_string` usually does not satisfy `&'static str`.

## Learning Scope

Already learned tools:

- ownership and borrowing
- basic lifetime syntax
- generic functions

New concept in this exercise:

- difference between `&'static T` and `T: 'static`

Out of scope (do not use yet):

- `Box::leak`
- global state (`static`, `OnceLock`)
- `unsafe`

## Constraints

- Keep code in `src/main.rs`.
- Do not use `unsafe`.
- Do not use extra crates.

## Review Focus

- Correct understanding of reference lifetime vs type bound.
- No hidden extra abstractions.
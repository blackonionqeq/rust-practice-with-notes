# 61 DST-Friendly API Boundaries

## Goal

Design function signatures that naturally accept both owned and borrowed data.

## Project

Create project:

```bash
cargo new practice/expert/61_dst_api_boundaries --name dst_api_boundaries
```

## Requirements

Define and use:

```rust
fn first_word(input: &str) -> &str
```

Return the first word (split by ASCII whitespace). If empty, return `""`.

```rust
fn sum_slice(values: &[i32]) -> i32
```

Return the sum of all items.

In `main`:

1. Call `first_word` with both `String` and string literal.
2. Call `sum_slice` with array slice and vector slice.
3. Print results.

## Learning Scope

Already learned tools:

- slices (`&str`, `&[T]`)
- iterator basics
- ownership and borrowing

New knowledge points in this exercise:

- choosing DST-friendly parameter types as API boundaries
- using `&str` instead of `&String` for read-only text
- using `&[T]` instead of `&Vec<T>` for read-only sequences

Out of scope (do not use yet):

- `?Sized` generic abstractions
- trait objects
- `unsafe`

## Constraints

- Keep code in `src/main.rs`.
- Do not use `unsafe`.
- Do not use extra crates.
- Keep implementation direct and readable.

## Review Focus

- API signature quality (`&str`, `&[T]` choices).
- Simplicity over abstraction.

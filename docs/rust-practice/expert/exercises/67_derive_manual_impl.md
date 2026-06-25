# 67 Derive vs Manual Impl

## Goal

Understand what `#[derive]` generates by writing the equivalent trait impls by hand first.

## Project

Create project:

```bash
cargo new practice/expert/67_derive_manual_impl --name derive_manual_impl
```

## Requirements

### Part A: Manual impls

Define struct **without** any `#[derive]`:

```rust
struct Point {
    x: f64,
    y: f64,
}
```

Manually implement:

1. `std::fmt::Debug` for `Point`.
   - Output format: `Point { x: 1.0, y: 2.0 }` (any consistent format is fine).
2. `PartialEq` for `Point`.
   - Two points are equal when both `x` and `y` are equal.

In `main`:

1. Create two `Point` values.
2. Print one with `{:?}`.
3. Compare two points with `==` and print the result.

Confirm it compiles and runs.

### Part B: Replace with derive

Add `#[derive(Debug, PartialEq)]` to the struct definition and **delete** both manual impls.

Verify the program still compiles and produces the same output.

Add a short comment (2-3 lines): what the derive attribute saved you from writing.

## Learning Scope

Already learned tools:

- struct and impl basics
- trait implementations
- `fmt::Debug` trait signature
- basic operator overloading via traits

New knowledge points in this exercise:

- derive as compile-time code generation of equivalent manual impls
- default field-by-field trait behavior
- field trait bounds required by derive

Out of scope (do not use yet):

- writing procedural macros
- `syn` / `quote`
- custom derive for non-standard traits

## Constraints

- Keep code in `src/main.rs`.
- Do not use extra crates.
- Part A must compile before adding derive.
- Part B final file should only keep the derive version.

## Review Focus

- Manual impl is correct before switching to derive.
- Understands derive is syntactic sugar, not magic.
- Comment clearly names the boilerplate derive replaced.

# 66 Derive Macro Concept

## Goal

Understand derive macros at usage level (conceptual), without implementing one.

## Project

Create project:

```bash
cargo new practice/expert/66_derive_macro_concept --name derive_macro_concept
```

## Requirements

Define struct:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}
```

In `main`:

1. Create one `User` value.
2. Clone it.
3. Compare original and clone with `==`.
4. Print with `{:?}`.
5. Add a short comment (2-3 lines): derive macro expands code at compile time.

## Learning Scope

Already learned tools:

- struct and impl basics
- trait basics
- formatting and equality checks

New concept in this exercise:

- what derive macro provides conceptually for everyday code reading

Out of scope (do not use yet):

- writing procedural macros
- `syn` / `quote`
- attribute parsing internals

## Constraints

- Keep code in `src/main.rs`.
- Do not use extra crates.
- Focus on understanding generated behavior, not macro internals.

## Review Focus

- Knows what behavior `derive` gives.
- Can explain why this is compile-time code generation.
# 63 macro_rules Basic

## Goal

Write and use your first tiny `macro_rules!` helper.

## Project

Create project:

```bash
cargo new practice/expert/63_macro_rules_basic --name macro_rules_basic
```

## Requirements

Define macro:

```rust
say_hello!(name)
```

Behavior:

- Accept one expression as name.
- Print `hello, {name}`.

In `main`:

1. Call `say_hello!("rust")`.
2. Call `say_hello!(String::from("macro"))`.

## Learning Scope

Already learned tools:

- functions
- formatting with `println!`
- expression basics

New knowledge points in this exercise:

- defining and invoking a minimal `macro_rules!`
- matching one expression with `$name:expr`
- understanding macro expansion before normal compilation

Out of scope (do not use yet):

- repetition patterns (`$()*`)
- complex matcher combinations
- procedural macros

## Constraints

- Keep code in `src/main.rs`.
- Do not use extra crates.
- Keep macro body short and readable.

## Review Focus

- Can correctly match one expression input.
- Invocation style is clear and consistent.

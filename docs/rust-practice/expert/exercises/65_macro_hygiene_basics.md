# 65 Macro Hygiene Basics

## Goal

Understand macro hygiene with a tiny variable shadowing-safe helper.

## Project

Create project:

```bash
cargo new practice/expert/65_macro_hygiene_basics --name macro_hygiene_basics
```

## Requirements

Define macro:

```rust
measure_len!(expr)
```

Behavior:

- Evaluate the input expression once.
- Store it in an internal temporary variable.
- Return its string length as `usize`.

In `main`:

1. Create `let value = String::from("rust");`
2. Print `measure_len!(value.clone())`.
3. Define local variable named `tmp` in `main` and show macro still works.

## Learning Scope

Already learned tools:

- local variables
- ownership and `clone`
- basic macro usage

New concept in this exercise:

- hygiene intuition: internal macro variable should not break caller scope

Out of scope (do not use yet):

- full hygiene formalism
- token-level advanced patterns
- procedural macros

## Constraints

- Keep code in `src/main.rs`.
- Do not use extra crates.
- Ensure input expression is not evaluated multiple times.

## Review Focus

- Caller variable names do not conflict with macro internals.
- Expression evaluation count is correct (once).
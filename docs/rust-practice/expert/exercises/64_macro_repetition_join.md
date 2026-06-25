# 64 Macro Repetition Join

## Goal

Practice repetition pattern in `macro_rules!` with a simple output string.

## Project

Create project:

```bash
cargo new practice/expert/64_macro_repetition_join --name macro_repetition_join
```

## Requirements

Define macro:

```rust
join_words!("a", "b", "c")
```

Behavior:

- Accept one or more string-like expressions separated by commas.
- Return a `String` joined by `-`.
- Example result: `"a-b-c"`.

In `main`:

1. Print `join_words!("rust")`.
2. Print `join_words!("macro", "rules", "ok")`.

## Learning Scope

Already learned tools:

- `String`
- iterating / collecting basics
- first macro definition and call

New concept in this exercise:

- repetition matcher (`$(...),+`) in `macro_rules!`

Out of scope (do not use yet):

- nested repetitions
- advanced token tree tricks
- procedural macros

## Constraints

- Keep code in `src/main.rs`.
- Do not use extra crates.
- Keep implementation straightforward (readability first).

## Review Focus

- Correct repetition matcher usage.
- Handles one item and multiple items correctly.
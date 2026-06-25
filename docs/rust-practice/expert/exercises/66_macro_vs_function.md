# 66 Macro vs Function

## Goal

Practice deciding when a macro is unnecessary.

## Project

Create project:

```bash
cargo new practice/expert/66_macro_vs_function --name macro_vs_function
```

## Requirements

1. First define a macro `to_kv_line!(k, v)` that returns `"k=v"` as `String`.
2. Then replace it with a normal function:

```rust
fn to_kv_line(k: &str, v: &str) -> String
```

3. In `main`, call the function for at least two pairs and print results.
4. Add a short comment: why function is better than macro here.

## Learning Scope

Already learned tools:

- string formatting
- functions
- basic macro usage

New concept in this exercise:

- practical rule for "when not to use macro"

Out of scope (do not use yet):

- benchmark-driven micro-optimization
- advanced generic metaprogramming
- procedural macros

## Constraints

- Keep code in `src/main.rs`.
- Do not use extra crates.
- Final submitted version should keep the function solution.

## Review Focus

- Decision quality: picks simpler tool when enough.
- Explanation is concrete, not slogan-style.

- Knows what behavior `derive` gives.
- Can explain why this is compile-time code generation.
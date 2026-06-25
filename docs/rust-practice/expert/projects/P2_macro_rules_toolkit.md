# P2 Macro Rules Toolkit

## Goal

Integrate E2 concepts: write two small but practical `macro_rules!` helpers, then justify the design choices.

This is an expert integration project after exercise 67.

## Project

Create project:

```bash
cargo new practice/expert/P2_macro_rules_toolkit --name macro_rules_toolkit
```

## Requirements

### Macro 1: `assert_contains!`

A debug-assertion macro that checks whether a string contains a substring, and prints a clear message on failure.

```rust
assert_contains!(haystack, needle)
```

Behavior:

- If `haystack` contains `needle`, do nothing.
- If not, panic with a message like: `assertion failed: "hello world" does not contain "rust"`.
- Both arguments should be evaluated once only.

### Macro 2: `log_fields!`

A variadic macro that formats one or more `name => value` pairs into a single `String`.

```rust
log_fields!(mode => "dev", region => "local", retry => 3)
// produces: "mode=dev region=local retry=3"
```

Behavior:

- Accept one or more `ident => expr` pairs separated by commas.
- Return a `String` with pairs joined by spaces in `key=value` format.

In `main`:

1. Call `assert_contains!("hello rust", "rust")` — should pass silently.
2. Call `log_fields!` with at least two pairs and print the result.
3. Add a short comment for each macro: is there a simpler function alternative? Why or why not?

## Constraints

- Keep all code in `src/main.rs`.
- Do not use extra crates.
- No procedural macros.
- Each macro body should be readable at a glance.
- Write at least one test for each macro.

## Review Focus

- `assert_contains!` evaluates both expressions exactly once.
- `log_fields!` correctly handles the repetition separator (no trailing space).
- Comments demonstrate understanding of when macros are justified vs when a function suffices.
- Tests cover both the happy path and (for `assert_contains!`) the failure path with `should_panic`.

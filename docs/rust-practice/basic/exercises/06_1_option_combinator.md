# 06.1 Option Combinator

## Goal

Practice converting `Result` to `Option` and using `Option` combinators.

## Project

Create project:

```bash
cargo new practice/basic/06_1_option_combinator --name option_combinator
```

## Required Function

```rust
fn parse_and_double(s: &str) -> Option<i32>
```

Return:

- `Some(number * 2)` when `s` can be parsed as `i32`.
- `None` when parsing fails.

Use:

- `.parse::<i32>()`
- `.ok()`
- `.map(...)`

## Required Function

```rust
fn print_result(s: &str)
```

Print:

- the doubled value for `Some(value)`
- `None` for `None`

## Main Program

In `main`:

1. Call `print_result("42")`.
2. Call `print_result("abc")`.
3. Call `print_result("0")`.

## Constraints

- Do not use `match` in `parse_and_double`.
- Do not use `if let` in `parse_and_double`.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use `?`.
- Use an `Option` method chain in `parse_and_double`.

## Review Result

Passed.

Implementation used:

```rust
s.parse::<i32>().ok().map(|n| n * 2)
```

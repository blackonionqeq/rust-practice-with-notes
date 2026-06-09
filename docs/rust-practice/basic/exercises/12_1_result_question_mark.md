# 12.1 Result Question Mark

## Goal

Practice using `?` to return early from a `Result`.

## Project

Create project:

```bash
cargo new practice/basic/12_1_result_question_mark --name result_question_mark
```

## Required Function

```rust
fn parse_number(input: &str) -> Result<i32, String>
```

Return:

- `Ok(number)` when `input` can be parsed as `i32`.
- `Err(String::from("invalid number"))` when parsing fails.

Use `match` on `input.parse::<i32>()`.

## Required Function

```rust
fn divide(a: i32, b: i32) -> Result<i32, String>
```

Return:

- `Ok(a / b)` when `b` is not `0`.
- `Err(String::from("cannot divide by zero"))` when `b` is `0`.

## Required Function

```rust
fn parse_and_divide(a: &str, b: &str) -> Result<i32, String>
```

In this function:

- Use `parse_number(a)?` to get the first number.
- Use `parse_number(b)?` to get the second number.
- Use `divide(first, second)?` to get the result.
- Return `Ok(result)`.

## Required Function

```rust
fn print_result(a: &str, b: &str)
```

Use `match` on `parse_and_divide(a, b)`:

- `Ok(value)` prints `result: {value}`.
- `Err(message)` prints `error: {message}`.

## Main Program

In `main`:

1. Call `print_result("10", "2")`.
2. Call `print_result("10", "0")`.
3. Call `print_result("abc", "2")`.

## Constraints

- Use `Result<i32, String>`.
- Use `?` inside `parse_and_divide`.
- Do not use `?` in `parse_number`, `divide`, or `print_result`.
- Use `match` in `parse_number` and `print_result`.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use `map_err`.

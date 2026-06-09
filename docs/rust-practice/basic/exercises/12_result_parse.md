# 12 Result Parse

## Goal

Practice basic error handling with `Result`, `Ok`, `Err`, and `match`.

## Project

Create project:

```bash
cargo new practice/basic/12_result_parse --name result_parse
```

## Required Function

```rust
fn parse_age(input: &str) -> Result<u32, String>
```

Return:

- `Ok(age)` when `input` can be parsed as `u32`.
- `Err(String::from("invalid age"))` when parsing fails.

Use `match` on `input.parse::<u32>()`.

## Required Function

```rust
fn print_age(input: &str)
```

Use `match` on `parse_age(input)`:

- `Ok(age)` prints `age: {age}`.
- `Err(message)` prints `error: {message}`.

## Main Program

In `main`:

1. Call `print_age("18")`.
2. Call `print_age("abc")`.
3. Call `print_age("0")`.

## Constraints

- Use `Result<u32, String>`.
- Use `Ok` and `Err`.
- Use `match` in both required functions.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use `?`.
- Do not use `map_err`.

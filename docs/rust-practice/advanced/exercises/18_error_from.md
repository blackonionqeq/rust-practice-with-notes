# 18 Error From

## Goal

Practice custom error types, `Display`, `From`, and `?` error conversion.

## Project

Create project:

```bash
cargo new practice/advanced/18_error_from --name error_from
```

## Required Error Type

Define:

```rust
#[derive(Debug)]
enum AppError {
    Parse(std::num::ParseIntError),
    Zero,
}
```

Implement:

```rust
impl std::fmt::Display for AppError
```

Required messages:

- `Parse(_)`: `invalid number`
- `Zero`: `cannot divide by zero`

Implement:

```rust
impl From<std::num::ParseIntError> for AppError
```

Convert parse errors into `AppError::Parse(error)`.

## Required Functions

```rust
fn parse_number(input: &str) -> Result<i32, AppError>
```

Use `input.parse::<i32>()?`.

```rust
fn divide(a: i32, b: i32) -> Result<i32, AppError>
```

Return `Err(AppError::Zero)` when `b == 0`.

```rust
fn parse_and_divide(a: &str, b: &str) -> Result<i32, AppError>
```

Use `?` to call `parse_number` and `divide`.

```rust
fn print_result(a: &str, b: &str)
```

Use `match` and print:

- `result: {value}`
- `error: {error}`

## Main Program

In `main`:

1. Call `print_result("10", "2")`.
2. Call `print_result("10", "0")`.
3. Call `print_result("abc", "2")`.

## Constraints

- Use the custom `AppError`.
- Implement `Display` manually.
- Implement `From<std::num::ParseIntError>` manually.
- Use `?` in `parse_number` and `parse_and_divide`.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use external crates.

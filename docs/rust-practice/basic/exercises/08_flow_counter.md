# 08 Flow Counter

## Goal

Practice basic flow control: `if`, `while`, `for`, `break`, and `continue`.

## Project

Create project:

```bash
cargo new practice/basic/08_flow_counter --name flow_counter
```

## Required Function

```rust
fn describe_number(n: i32)
```

Use `if` / `else if` / `else`:

- Print `{n} is positive` when `n > 0`.
- Print `{n} is zero` when `n == 0`.
- Print `{n} is negative` otherwise.

## Required Function

```rust
fn count_down(start: i32)
```

Use a `while` loop:

- Print numbers from `start` down to `1`.
- Print `done` after the loop.

## Required Function

```rust
fn print_odd_numbers()
```

Use a `for` loop over `1..=10`:

- Skip even numbers with `continue`.
- Print odd numbers.

## Main Program

In `main`:

1. Call `describe_number(7)`.
2. Call `describe_number(0)`.
3. Call `describe_number(-3)`.
4. Call `count_down(3)`.
5. Call `print_odd_numbers()`.

## Constraints

- Use `if` / `else if` / `else` in `describe_number`.
- Use `while` in `count_down`.
- Use `for` in `print_odd_numbers`.
- Use `continue` to skip even numbers.
- Do not use `match`.
- Do not use iterator methods like `.filter()` or `.for_each()`.

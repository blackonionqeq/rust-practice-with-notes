# 13 Lifetime Longest

## Goal

Practice a basic lifetime annotation on function parameters and return values.

## Project

Create project:

```bash
cargo new practice/basic/13_lifetime_longest --name lifetime_longest
```

## Required Function

```rust
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str
```

Return:

- `first` when `first.len() >= second.len()`.
- `second` otherwise.

## Required Function

```rust
fn print_longest(first: &str, second: &str)
```

In this function:

- Call `longest(first, second)`.
- Print `longest: {result}`.

## Main Program

In `main`:

1. Create `String::from("rust")`.
2. Create `String::from("ownership")`.
3. Call `print_longest` with references to both strings.
4. Create two string slices:

```rust
let short = "abc";
let long = "abcdef";
```

5. Call `print_longest(short, long)`.

## Constraints

- Use the exact lifetime signature on `longest`.
- Return a string slice, not a new `String`.
- Do not use `clone`.
- Do not use `to_string`.
- Do not use `String::from` inside `longest`.
- Do not return a reference to a local variable created inside `longest`.

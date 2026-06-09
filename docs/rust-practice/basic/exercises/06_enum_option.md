# 06 Enum Option

## Goal

Practice basic `enum`, `Option`, and `match`.

## Project

Create project:

```bash
cargo new practice/basic/06_enum_option --name enum_option
```

## Required Enum

Define:

```rust
#[derive(Debug)]
enum LoginStatus {
    Guest,
    User(String),
}
```

## Required Functions

```rust
fn display_status(status: LoginStatus)
```

Use `match`:

- `Guest` prints `guest user`.
- `User(name)` prints `logged in as {name}`.

```rust
fn find_user(id: u32) -> Option<String>
```

Return:

- `Some(String::from("black"))` when `id` is `1`.
- `Some(String::from("rustacean"))` when `id` is `2`.
- `None` for any other id.

## Main Program

In `main`:

1. Create a `LoginStatus::Guest` and pass it to `display_status`.
2. Create a `LoginStatus::User(String::from("black"))` and pass it to `display_status`.
3. Call `find_user(1)` and use `match` to print:
   - `found user: {name}` for `Some(name)`.
   - `user not found` for `None`.
4. Call `find_user(99)` and use `match` to print the same two cases.

## Constraints

- Use `match` explicitly.
- Do not use `unwrap`.
- Do not use `map`, `and_then`, or `?`.
- Do not use `clone`.

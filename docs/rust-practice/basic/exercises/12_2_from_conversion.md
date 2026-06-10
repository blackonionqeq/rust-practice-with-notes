# 12.2 From Conversion

## Goal

Practice implementing a standard library trait with `impl From<T> for U`.

This exercise is a bridge between:

- custom trait implementations like `impl Summary for Article`
- later error conversion like `impl From<ParseIntError> for AppError`

## Project

Create project:

```bash
cargo new practice/basic/12_2_from_conversion --name from_conversion
```

## Required Struct

Define:

```rust
struct Age {
    years: u32,
}
```

## Required Implementation

Implement:

```rust
impl From<u32> for Age
```

The `from` method should return:

```rust
Age { years: value }
```

## Required Function

```rust
fn describe_age(age: Age) -> String
```

Return:

```text
age: {years}
```

## Main Program

In `main`:

1. Create one `Age` with `Age::from(18)`.
2. Create one `Age` with `20u32.into()`.
3. Print both values with `describe_age`.

## Constraints

- Use `impl From<u32> for Age`.
- Do not define your own trait.
- Do not use `clone`.
- Do not use `Copy`.
- Do not implement `Display`.

# 09 Match Grade

## Goal

Practice basic pattern matching with `match`: exact values, ranges, `_`, and `Option`.

## Project

Create project:

```bash
cargo new practice/basic/09_match_grade --name match_grade
```

## Required Function

```rust
fn grade(score: i32) -> &'static str
```

Use `match`:

- `90..=100` returns `"A"`.
- `80..=89` returns `"B"`.
- `70..=79` returns `"C"`.
- `60..=69` returns `"D"`.
- `0..=59` returns `"F"`.
- Anything else returns `"invalid"`.

## Required Function

```rust
fn describe_grade(score: Option<i32>)
```

Use `match`:

- `Some(value)` prints `score: {value}, grade: {grade}`.
- `None` prints `no score`.

Call the `grade` function inside the `Some(value)` branch.

## Main Program

In `main`:

1. Call `describe_grade(Some(95))`.
2. Call `describe_grade(Some(82))`.
3. Call `describe_grade(Some(58))`.
4. Call `describe_grade(Some(120))`.
5. Call `describe_grade(None)`.

## Constraints

- Use `match` in both required functions.
- Use range patterns like `90..=100`.
- Use `_` for the fallback branch in `grade`.
- Do not use `if` / `else if` in `grade`.
- Do not use `unwrap`.
- Do not use `map`, `and_then`, or `?`.

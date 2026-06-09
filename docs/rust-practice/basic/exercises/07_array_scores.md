# 07 Array Scores

## Goal

Practice fixed-size arrays, index access, array length, and simple `for` loops.

## Project

Create project:

```bash
cargo new practice/basic/07_array_scores --name array_scores
```

## Required Function

```rust
fn print_scores(scores: [i32; 5])
```

In this function:

- Print the first score with index access.
- Print the last score with index access.
- Print the array length with `.len()`.
- Use a `for` loop to print every score.

## Required Function

```rust
fn sum_scores(scores: [i32; 5]) -> i32
```

Return the total of all scores.

## Main Program

In `main`:

1. Create this array:

```rust
let scores = [80, 92, 75, 88, 100];
```

2. Pass `scores` to `print_scores`.
3. Pass `scores` to `sum_scores`.
4. Print `total: {total}`.

## Constraints

- Use the type `[i32; 5]` in both function signatures.
- Use index access at least twice, for the first and last score.
- Use a `for` loop in `print_scores`.
- Do not use `Vec`.
- Do not use iterator methods like `.iter().sum()`.
- Do not use `clone`.

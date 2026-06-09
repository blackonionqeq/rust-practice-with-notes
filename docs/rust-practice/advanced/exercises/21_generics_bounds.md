# 21 Generics Bounds

## Goal

Practice generic functions, trait bounds, and `where` clauses.

## Project

Create project:

```bash
cargo new practice/advanced/21_generics_bounds --name generics_bounds
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Pair<T> {
    left: T,
    right: T,
}
```

## Required Functions

```rust
fn larger<T>(left: T, right: T) -> T
where
    T: PartialOrd,
```

Return the larger value. If values are equal, return `left`.

```rust
fn describe_pair<T>(pair: &Pair<T>) -> String
where
    T: std::fmt::Display,
```

Return:

```text
left: {left}, right: {right}
```

```rust
fn larger_pair_value<T>(pair: Pair<T>) -> T
where
    T: PartialOrd,
```

Return the larger field from the pair.

## Main Program

In `main`:

1. Call `larger(3, 7)` and print the result.
2. Call `larger("rust", "go")` and print the result.
3. Create `Pair { left: 10, right: 8 }`.
4. Print `describe_pair(&pair)`.
5. Print `larger_pair_value(pair)`.

## Constraints

- Use generics.
- Use `where` clauses for trait bounds.
- Use `PartialOrd` for comparisons.
- Use `Display` for formatting.
- Do not use `clone`.
- Do not use `Copy` as a bound.

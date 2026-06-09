# 30 RefCell Counter

## Goal

Practice `RefCell<T>` and interior mutability.

## Project

Create project:

```bash
cargo new practice/advanced/30_refcell_counter --name refcell_counter
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Counter {
    value: std::cell::RefCell<i32>,
}
```

## Required Impl

Implement:

```rust
impl Counter {
    fn new() -> Counter
    fn increment(&self)
    fn add(&self, amount: i32)
    fn current(&self) -> i32
}
```

Behavior:

- `new` starts at `0`.
- `increment` adds `1`.
- `add` adds `amount`.
- `current` returns the current value.

## Main Program

In `main`:

1. Create a non-mutable binding: `let counter = Counter::new();`
2. Call `increment`.
3. Call `add(5)`.
4. Print `current: {value}`.
5. Print the counter with `Debug`.

## Constraints

- The binding in `main` must not be `mut`.
- Mutating methods must take `&self`.
- Use `borrow_mut` for mutation.
- Use `borrow` or dereference `borrow()` for reading.
- Do not use `Cell`.
- Do not use `Rc`.

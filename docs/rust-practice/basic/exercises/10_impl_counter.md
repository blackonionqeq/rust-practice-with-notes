# 10 Impl Counter

## Goal

Practice methods with `impl`, `self`, `&self`, and `&mut self`.

## Project

Create project:

```bash
cargo new practice/basic/10_impl_counter --name impl_counter
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Counter {
    value: i32,
}
```

## Required Impl

Create an `impl Counter` block with these methods:

```rust
fn new() -> Counter
```

Return a `Counter` with `value` set to `0`.

```rust
fn current(&self) -> i32
```

Return the current `value`.

```rust
fn increment(&mut self)
```

Add `1` to `value`.

```rust
fn add(&mut self, amount: i32)
```

Add `amount` to `value`.

```rust
fn reset(&mut self)
```

Set `value` back to `0`.

## Main Program

In `main`:

1. Create a mutable counter with `Counter::new()`.
2. Print `current: {value}` using `current()`.
3. Call `increment()` twice.
4. Call `add(5)`.
5. Print `current: {value}` again.
6. Call `reset()`.
7. Print the whole counter with `Debug`.

## Constraints

- Use an `impl Counter` block.
- Use `Counter::new()` to create the value.
- Use `&self` for `current`.
- Use `&mut self` for methods that change `value`.
- Do not access `counter.value` directly in `main`, except through `Debug` printing.
- Do not use `clone`.

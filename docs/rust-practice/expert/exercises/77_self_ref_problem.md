# 77 Self-Referential Struct Problem

## Goal

Understand why a struct cannot safely hold a reference to its own field, and learn the index-based workaround.

## Project

Create project:

```bash
cargo new practice/expert/77_self_ref_problem --name self_ref_problem
```

## Requirements

### Part A: Observe the compiler error

Try to define a struct that holds a reference into its own `data` field:

```rust
struct SelfRef<'a> {
    data: String,
    view: &'a str,  // intended to point into data
}
```

Then attempt to write a constructor:

```rust
impl<'a> SelfRef<'a> {
    fn new(s: String) -> Self {
        // try to set view = &s[..] here
        todo!()
    }
}
```

You do not need to make this compile. Read the compiler error carefully and add a comment explaining in your own words what the lifetime problem is.

### Part B: Index-based workaround

Instead, implement:

```rust
struct TokenView {
    data: String,
    start: usize,
    end: usize,
}

impl TokenView {
    fn new(data: String, start: usize, end: usize) -> Self
    fn view(&self) -> &str  // returns &self.data[start..end]
}
```

In `main`, create one `TokenView` and print its `view()` result.

## Learning Scope

Already learned tools:

- struct definitions and lifetimes
- string slicing

New concept in this exercise:

- why self-referential references are rejected by the borrow checker
- index-based workaround as the simplest safe alternative

Out of scope (do not use yet):

- `Pin`
- `Rc<RefCell<_>>`
- raw pointer self-reference (that is exercise 78)

## Constraints

- Keep code in `src/main.rs`.
- Part A is intentionally broken — comment out or use `todo!()`.
- No extra crates.

## Review Focus

- Comment in Part A explains the lifetime issue in concrete terms.
- Part B compiles and works correctly.

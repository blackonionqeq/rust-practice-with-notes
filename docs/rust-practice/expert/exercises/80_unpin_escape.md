# 80 Unpin Escape

## Goal

Show that `Pin` places no additional restriction on types that implement `Unpin`.

## Project

Create project:

```bash
cargo new practice/expert/80_unpin_escape --name unpin_escape
```

## Requirements

1. Create a `Pin<Box<i32>>`:
   ```rust
   let mut pinned = Box::pin(42i32);
   ```
2. Use `Pin::as_mut` and `Pin::get_mut` to get a `&mut i32` and modify the value.
3. Print the modified value.
4. Add a comment: why does `get_mut` work here but would not work for a `!Unpin` type like `Unmovable` from exercise 79?

Then add a short check using the `std::marker::Unpin` trait bound to verify that `i32` implements `Unpin`:

```rust
fn requires_unpin<T: Unpin>(_: T) {}
requires_unpin(0i32);  // compiles fine
```

Add a comment: what would happen if you called `requires_unpin` with an `Unmovable` from exercise 79?

## Learning Scope

Already learned tools:

- `Pin<Box<T>>` basics (79)
- trait bounds

New knowledge points in this exercise:

- `Unpin` means "safe to move even after pinning" — `Pin` adds no restriction for `Unpin` types
- almost all standard types implement `Unpin` automatically
- `Pin::get_mut` is safe only when `T: Unpin`
- `get_unchecked_mut` is the unsafe escape hatch for `!Unpin` types

Out of scope (do not use yet):

- `pin_project` crate
- `Pin` projection into struct fields

## Constraints

- Keep code in `src/main.rs`.
- No extra crates.
- The `requires_unpin(Unmovable {...})` call must be a comment.

## Review Focus

- `get_mut` is used correctly to mutate through the pin.
- Comment clearly explains why `get_mut` is available for `Unpin` types.

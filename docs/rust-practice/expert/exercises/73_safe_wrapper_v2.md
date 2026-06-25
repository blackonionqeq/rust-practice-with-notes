# 73 Safe Wrapper v2

## Goal

Extend the safe wrapper to enforce two invariants and add focused unit tests.

## Project

Create project:

```bash
cargo new practice/expert/73_safe_wrapper_v2 --name safe_wrapper_v2
```

## Requirements

Define:

```rust
pub fn read_at(data: &[i32], index: usize) -> Option<i32>
```

Inside the function:

1. Return `None` if `data` is empty or `index >= data.len()`.
2. Get a `*const i32` from `data.as_ptr()`.
3. Add a `// Safety:` comment listing **both** invariants that are now guaranteed.
4. Use `ptr.add(index)` and dereference inside an `unsafe` block.
5. Return `Some(val)`.

Write three unit tests in a `#[cfg(test)]` module:

1. Reading at a valid in-bounds index returns the correct value.
2. Reading at an out-of-bounds index returns `None`.
3. Reading from an empty slice returns `None`.

In `main`:

1. Call `read_at` for one valid and one out-of-bounds case and print both results.

## Learning Scope

Already learned tools:

- safe wrapper pattern (72)
- unit tests with `#[cfg(test)]`

New concept in this exercise:

- managing multiple invariants in one safe wrapper
- testing unsafe internals through the safe public interface

Out of scope (do not use yet):

- `NonNull<T>`
- `slice::get_unchecked`

## Constraints

- `read_at` must be a safe function (no `unsafe fn` in the signature).
- `// Safety:` comment must list both invariants explicitly.
- Keep code in `src/main.rs`.
- No extra crates.

## Review Focus

- Both invariants are named in the `// Safety:` comment.
- Tests cover valid index, out-of-bounds, and empty slice.
- `ptr.add(index)` is only reached after both checks pass.

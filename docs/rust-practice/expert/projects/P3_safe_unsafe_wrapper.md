# P3 Safe Unsafe Wrapper

## Goal

Integrate Stage 7 concepts into one small but complete abstraction: a safe public API that wraps raw pointer operations, with explicit invariants, safety contracts, and focused tests.

This is an expert integration project after exercise 74.

## Project

Create project:

```bash
cargo new practice/expert/P3_safe_unsafe_wrapper --name safe_unsafe_wrapper
```

## Requirements

Implement a `SliceIndex` helper that provides safe indexed access to a `&[i32]` slice using raw pointer arithmetic internally.

### API to implement

```rust
pub struct SliceIndex<'a> {
    // internal fields: raw pointer and length
}

impl<'a> SliceIndex<'a> {
    pub fn new(data: &'a [i32]) -> Self

    pub fn get(&self, index: usize) -> Option<i32>

    pub fn get_pair(&self, i: usize, j: usize) -> Option<(i32, i32)>

    pub fn swap_copy(&self, i: usize, j: usize) -> Option<[i32; 2]>
    // Returns [data[j], data[i]] — swapped values, does not mutate
}
```

### Requirements

- `new`: store a raw `*const i32` pointer and the slice length. Do not store the reference directly.
- `get`: bounds-check `index`, then dereference via raw pointer. Include a `// Safety:` comment.
- `get_pair`: bounds-check both indices independently, then read both. Include a `// Safety:` comment with two invariants listed.
- `swap_copy`: bounds-check both indices, return `[data[j], data[i]]` without modifying the original slice.
- Add a `PhantomData<&'a i32>` field to tie the lifetime to the input slice.

### Tests

Write at least four unit tests in a `#[cfg(test)]` module:

1. `get` returns `Some` for a valid index.
2. `get` returns `None` for an out-of-bounds index.
3. `get_pair` returns correct values for two valid indices.
4. `swap_copy` returns the values in swapped order.

In `main`, demonstrate all three methods with at least one call each.

## Constraints

- Keep all code in `src/main.rs`.
- Do not use extra crates.
- Every `unsafe` block must have a `// Safety:` comment.
- `unsafe` blocks must be minimal (no safe operations inside them).
- No `unsafe fn` in the public API — all public methods must be safe.

## Review Focus

- `PhantomData<&'a i32>` is present and correctly typed.
- Every `unsafe` block has a `// Safety:` comment that names the invariant.
- All four tests pass.
- No public `unsafe` surface.
- `unsafe` blocks are as small as possible (74-style discipline).

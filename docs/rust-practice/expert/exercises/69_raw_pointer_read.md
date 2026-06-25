# 69 Raw Pointer Read

## Goal

Create a raw pointer from a reference and dereference it inside an unsafe block.

## Project

Create project:

```bash
cargo new practice/expert/69_raw_pointer_read --name raw_pointer_read
```

## Requirements

Define:

```rust
fn first_byte(data: &[u8]) -> u8
```

Inside the function:

1. Get a `*const u8` using `data.as_ptr()`.
2. Add a comment directly above the `unsafe` block stating the precondition this function requires the caller to satisfy.
3. Dereference the pointer inside the `unsafe` block.
4. Return the byte value.

In `main`:

1. Call `first_byte` with a non-empty byte slice and print the result.
2. Add a comment explaining why calling it with an empty slice would be unsafe (do not actually call it with an empty slice).

## Learning Scope

Already learned tools:

- slices and references
- the five unsafe capabilities (68)

New knowledge points in this exercise:

- constructing and dereferencing a `*const T` in the minimal, single-operation case
- deriving raw pointers from valid references
- documenting the non-empty precondition before dereference

Out of scope (do not use yet):

- safe wrappers with `Option` return type (that is exercise 72)
- pointer arithmetic with `.add(n)` (that is exercise 73)

## Constraints

- Keep code in `src/main.rs`.
- One `unsafe` block, one dereference.
- No extra crates.

## Review Focus

- Precondition comment is present and accurate.
- Pointer is derived from a valid reference, not cast from an integer.

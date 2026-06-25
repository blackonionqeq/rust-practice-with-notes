# 72 Safe Wrapper v1

## Goal

Wrap a single raw pointer dereference behind a safe public function that enforces one invariant at the boundary.

## Project

Create project:

```bash
cargo new practice/expert/72_safe_wrapper_v1 --name safe_wrapper_v1
```

## Requirements

Define:

```rust
pub fn first_byte(data: &[u8]) -> Option<u8>
```

Inside the function:

1. Return `None` if `data` is empty.
2. Get a `*const u8` from `data.as_ptr()`.
3. Add a `// Safety:` comment stating the invariant that is now guaranteed by the check above.
4. Dereference the pointer inside an `unsafe` block.
5. Return `Some(byte)`.

In `main`:

1. Call `first_byte` with a non-empty byte slice and print the result.
2. Call `first_byte` with an empty slice and print the result.

## Learning Scope

Already learned tools:

- raw pointer read (69)
- `unsafe fn` contract pattern (71)
- `Option` return type

New knowledge points in this exercise:

- the safe wrapper pattern: enforce invariants at the safe boundary, then enter `unsafe` with the guarantee already established
- using `Option` to represent rejected unsafe preconditions
- keeping callers on a safe API

Out of scope (do not use yet):

- multiple invariants (that is exercise 73)
- pointer arithmetic

## Constraints

- `first_byte` must be a safe function (no `unsafe fn` in the signature).
- `// Safety:` comment is required directly before the `unsafe` block.
- Keep code in `src/main.rs`.
- No extra crates.

## Review Focus

- The invariant is checked before entering `unsafe`.
- `// Safety:` comment references the specific check that was just performed.
- Function signature is safe — callers need no special knowledge.

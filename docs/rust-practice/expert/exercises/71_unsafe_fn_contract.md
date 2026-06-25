# 71 Unsafe Function Contract

## Goal

Define an `unsafe fn` with a complete `# Safety` doc comment, then demonstrate one correct and one conceptually incorrect call site.

## Project

Create project:

```bash
cargo new practice/expert/71_unsafe_fn_contract --name unsafe_fn_contract
```

## Requirements

Define:

```rust
/// # Safety
///
/// (fill in at least two preconditions)
unsafe fn read_i32(ptr: *const i32) -> i32 {
    *ptr
}
```

Complete the `# Safety` doc comment with at least two concrete preconditions (for example: non-null, points to valid initialized memory, not aliased by a mutable reference).

In `main`:

1. **Correct call**: derive a `*const i32` from a live stack-allocated `i32`, call `read_i32` inside `unsafe { }`, and print the result.
2. **Incorrect call (comment only)**: add a commented-out code block showing a call that would violate the contract, and name which precondition it breaks.

## Learning Scope

Already learned tools:

- raw pointer read and write (69, 70)
- `unsafe` block syntax

New concept in this exercise:

- `unsafe fn` as a contract declaration: the caller bears the burden of satisfying the preconditions

Out of scope (do not use yet):

- wrapping `unsafe fn` inside a safe function (that is exercise 72)
- `NonNull<T>`

## Constraints

- Keep code in `src/main.rs`.
- `# Safety` must list at least two concrete preconditions, not vague statements.
- No extra crates.

## Review Focus

- Safety doc is specific enough to guide a future caller.
- Correct call site derives the pointer from a valid live variable.
- Incorrect example comment names a concrete precondition violation.

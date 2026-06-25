# 70 Raw Pointer Write

## Goal

Write through a `*mut T` raw pointer and state the invariant the code relies on.

## Project

Create project:

```bash
cargo new practice/expert/70_raw_pointer_write --name raw_pointer_write
```

## Requirements

Define:

```rust
fn negate_first(data: &mut [i32])
```

Inside the function:

1. Return early if `data` is empty.
2. Get a `*mut i32` using `data.as_mut_ptr()`.
3. Add a `// Safety:` comment before the `unsafe` block stating exactly one invariant.
4. Inside the `unsafe` block, read the current value and write back its negation.

In `main`:

1. Create a `Vec<i32>` with at least three elements.
2. Call `negate_first`.
3. Print the full vec to confirm only the first element changed.

## Learning Scope

Already learned tools:

- mutable references and slices
- raw pointer read (69)

New knowledge points in this exercise:

- writing through `*mut T` and explicitly stating the write invariant
- using `&mut [T]` as the source of exclusive access
- checking element existence before raw pointer write

Out of scope (do not use yet):

- pointer arithmetic with `.add(n)`
- multiple writes in one unsafe block

## Constraints

- Keep code in `src/main.rs`.
- One `unsafe` block, one write operation.
- No extra crates.

## Review Focus

- `// Safety:` comment names the invariant concretely (not just "it is safe").
- Only the first element is modified.

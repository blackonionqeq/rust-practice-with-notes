# 78 Move Invalidates Raw Pointer

## Goal

Demonstrate concretely why moving a value can invalidate a raw pointer that pointed into it.

## Project

Create project:

```bash
cargo new practice/expert/78_move_invalidates_ref --name move_invalidates_ref
```

## Requirements

Define a struct that stores a raw pointer intended to point to one of its own fields:

```rust
struct Fragile {
    data: [i32; 3],
    ptr: *const i32,  // intended to point to data[0]
}
```

Implement:

```rust
impl Fragile {
    fn new(values: [i32; 3]) -> Self {
        // Create the struct first, then set ptr = data.as_ptr()
        // (Note: this is already broken by design — see exercise)
    }

    // Safety: only call this immediately after new(), before any move
    unsafe fn read_ptr(&self) -> i32 {
        unsafe { *self.ptr }
    }
}
```

In `main`:

1. Call `Fragile::new` and print the address of `data[0]` and the value of `ptr` immediately (they should match at this point).
2. Move the struct to a new binding: `let b = a;`
3. Print the address of `b.data[0]` and the stored `b.ptr`.
4. Add a comment: are the two addresses the same after the move? What does this mean for `b.ptr`?

Do not call `read_ptr` after the move. Add a comment explaining why that would be UB.

## Learning Scope

Already learned tools:

- raw pointer basics (69-70)
- struct definitions

New knowledge points in this exercise:

- a move copies the struct's bytes to a new address, leaving the raw pointer pointing to the old (now invalid) location
- raw pointers do not let the borrow checker track address validity
- pointers into inline fields behave differently from pointers into separately allocated heap data
- unsafe operations inside `unsafe fn` still need an explicit `unsafe` block in Rust 2024 style

Out of scope (do not use yet):

- `Pin` (that is exercise 79)
- fixing the problem (this exercise is about understanding it)

## Constraints

- Keep code in `src/main.rs`.
- No extra crates.
- Print both addresses; the comparison is the learning goal.
- Do not call `read_ptr` after the move.

## Review Focus

- Addresses are printed both before and after the move.
- Comment correctly identifies whether they match and what that implies.
- `read_ptr` is not called after the move.

# 79 Pin Concept

## Goal

Use `Pin<Box<T>>` with `PhantomPinned` to create a value that the type system prevents from being moved.

## Project

Create project:

```bash
cargo new practice/expert/79_pin_concept --name pin_concept
```

## Requirements

Define:

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;

struct Unmovable {
    data: String,
    _pin: PhantomPinned,
}
```

In `main`:

1. Create a pinned heap-allocated value:
   ```rust
   let pinned: Pin<Box<Unmovable>> = Box::pin(Unmovable {
       data: String::from("hello"),
       _pin: PhantomPinned,
   });
   ```
2. Access the `data` field through the pin using `pinned.data` (via `Deref`) and print it.
3. Try to move it out with `let moved = *pinned;` — add this as a **commented-out** line with a comment explaining the compile error you would get.
4. Add a 2-3 line comment: what does `PhantomPinned` do, and what does `Pin` prevent?

## Learning Scope

Already learned tools:

- `Box<T>` and heap allocation
- raw pointer / move mechanics (78)

New concept in this exercise:

- `PhantomPinned` opts a type out of `Unpin`
- `Pin<P>` prevents moving the value behind the pointer

Out of scope (do not use yet):

- `pin!` macro (stack pinning)
- async / `Future` internals
- manual `Pin` projection

## Constraints

- Keep code in `src/main.rs`.
- No extra crates.
- The move-out attempt must be a comment, not compiled code.

## Review Focus

- `Box::pin` is used correctly.
- `PhantomPinned` is present in the struct.
- Comment on the move-out correctly names the compile error reason.

# 75 repr(C) Struct Layout

## Goal

Understand what `#[repr(C)]` guarantees about struct field layout, and write a minimal `extern "C"` function declaration.

## Project

Create project:

```bash
cargo new practice/expert/75_repr_c_struct --name repr_c_struct
```

## Requirements

Define two structs with the same fields, one with `#[repr(C)]` and one without:

```rust
#[repr(C)]
struct PointC {
    x: f32,
    y: f32,
    tag: u8,
}

struct PointRust {
    x: f32,
    y: f32,
    tag: u8,
}
```

In `main`:

1. Print `std::mem::size_of::<PointC>()` and `std::mem::size_of::<PointRust>()`.
2. Print `std::mem::offset_of!(PointC, x)`, `offset_of!(PointC, y)`, `offset_of!(PointC, tag)`.
3. Add a comment explaining why the two structs may have different sizes.

Then add an `extern "C"` declaration block (no implementation, just a type-level boundary):

```rust
extern "C" {
    fn add_floats(a: f32, b: f32) -> f32;
}
```

Add a comment: what would happen if you called `add_floats` without a matching C definition linked in.

Do not call `add_floats` from `main`.

## Learning Scope

Already learned tools:

- struct definitions
- `std::mem::size_of`

New concept in this exercise:

- `#[repr(C)]` as a layout guarantee for FFI boundaries
- `extern "C"` block as a declaration of a foreign function (type-level only)

Out of scope (do not use yet):

- actually linking to C code
- `build.rs`
- `#[link]` attribute

## Constraints

- Keep code in `src/main.rs`.
- Do not call `add_floats` (it has no implementation).
- No extra crates.
- `offset_of!` is in `std::mem::offset_of!` (stable since Rust 1.77).

## Review Focus

- Correct understanding of why `#[repr(C)]` matters at type boundaries.
- `extern "C"` block is present but the function is not called.
- Comment explains the layout difference, not just restates it.

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
struct DemoC {
    a: u8,
    b: u64,
    c: u8,
}

struct DemoRust {
    a: u8,
    b: u64,
    c: u8,
}
```

In `main`:

1. Print `std::mem::size_of::<DemoC>()` and `std::mem::size_of::<DemoRust>()`.
2. Print the offsets of `b` and `c` in both structs with `std::mem::offset_of!`.
3. Add a comment explaining why `DemoC` is commonly 24 bytes on a 64-bit target while the current Rust compiler may make `DemoRust` 16 bytes.
4. The comment must also state that the observed `DemoRust` layout is not a stable layout guarantee.

Then add an `unsafe extern "C"` declaration block (no implementation, just a type-level boundary):

```rust
unsafe extern "C" {
    fn add_bytes(a: u8, b: u8) -> u8;
}
```

Add a comment: what would happen if you called `add_bytes` without a matching C definition linked in.

Do not call `add_bytes` from `main`.

## Learning Scope

Already learned tools:

- struct definitions
- `std::mem::size_of`

New knowledge points in this exercise:

- `#[repr(C)]` as a layout guarantee for FFI boundaries
- why equal field sets can still have different layouts when Rust is allowed to reorder fields
- field offset, alignment, and tail padding as reasons `size_of` may exceed field sizes
- `unsafe extern "C"` block as a declaration of a foreign function (type-level only)
- why Rust 2024 marks foreign function declarations as unsafe to declare and unsafe to call

Out of scope (do not use yet):

- actually linking to C code
- `build.rs`
- `#[link]` attribute

## Constraints

- Keep code in `src/main.rs`.
- Do not call `add_bytes` (it has no implementation).
- No extra crates.
- `offset_of!` is in `std::mem::offset_of!` (stable since Rust 1.77).

## Review Focus

- Correct understanding of why `#[repr(C)]` matters at type boundaries.
- Both size and field offsets are compared; the current default Rust layout is not described as guaranteed.
- `extern "C"` block is present but the function is not called.
- Comment explains the layout difference, not just restates it.

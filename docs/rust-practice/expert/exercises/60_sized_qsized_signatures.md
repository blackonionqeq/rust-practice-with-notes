# 60 Sized and ?Sized Signatures

## Goal

Practice API signatures that accept dynamically sized values via references.

## Project

Create project:

```bash
cargo new practice/expert/60_sized_qsized_signatures --name sized_qsized_signatures
```

## Requirements

Define and use:

```rust
fn byte_len<T: ?Sized + AsRef<[u8]>>(value: &T) -> usize
```

Return `value.as_ref().len()`.

In `main`:

1. Call `byte_len` with `String`.
2. Call `byte_len` with `&str`.
3. Call `byte_len` with `Vec<u8>`.
4. Call `byte_len` with slice `&[u8]`.
5. Print all results.

## Learning Scope

Already learned tools:

- generics and trait bounds
- references and slices
- `String`, `Vec<u8>`

New concept in this exercise:

- `?Sized` on generic parameters

Out of scope (do not use yet):

- trait objects (`dyn Trait`)
- custom DST types
- `unsafe`

## Constraints

- Keep code in `src/main.rs`.
- Do not use `unsafe`.
- Do not use extra crates.
- Do not add additional generic helper layers.

## Review Focus

- Correct `?Sized` usage with reference parameter.
- Understanding that DST values are handled behind references.
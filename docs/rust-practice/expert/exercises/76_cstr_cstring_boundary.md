# 76 CStr and CString Boundary

## Goal

Practice the safe boundary between Rust strings and C-style null-terminated strings.

## Project

Create project:

```bash
cargo new practice/expert/76_cstr_cstring_boundary --name cstr_cstring_boundary
```

## Requirements

Use `std::ffi::{CStr, CString}`.

1. Create a `CString` from a normal string literal `"hello"`. Handle the `Result` (use `expect` for this exercise).
2. Get a `&CStr` from the `CString`.
3. Convert the `&CStr` back to `&str` using `to_str()`. Handle the `Result`.
4. Print the `&str` result.
5. Attempt to create a `CString` from `"hel\0lo"` (a string with an embedded null byte). Print the error rather than panicking.

Add a short comment (2-3 lines): why C strings cannot contain interior null bytes, and what the `\0` terminator means.

## Learning Scope

Already learned tools:

- `Result` and error handling
- string basics

New concept in this exercise:

- `CString` / `&CStr` as the safe boundary type for null-terminated strings
- why embedded null bytes are invalid in C strings

Out of scope (do not use yet):

- passing `CString` across an actual FFI call
- `unsafe` (all conversions in this exercise are safe)

## Constraints

- Keep code in `src/main.rs`.
- Do not use `unsafe`.
- No extra crates.

## Review Focus

- `CString::new` error is handled, not unwrapped unconditionally.
- Embedded-null case prints the error instead of panicking.
- Comment explains the null-terminator rule clearly.

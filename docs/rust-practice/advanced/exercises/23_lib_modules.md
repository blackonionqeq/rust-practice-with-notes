# 23 Lib Modules

## Goal

Practice splitting a project into `lib.rs`, `main.rs`, and modules.

## Project

Create project:

```bash
cargo new practice/advanced/23_lib_modules --name lib_modules
```

## Required Files

Create:

```text
src/lib.rs
src/stats.rs
src/format.rs
```

Keep the existing:

```text
src/main.rs
```

## Required Module Layout

In `src/lib.rs`:

```rust
pub mod format;
pub mod stats;
```

In `src/stats.rs`, define:

```rust
#[derive(Debug)]
pub struct Summary {
    pub lines: usize,
    pub words: usize,
}
```

Define:

```rust
pub fn summarize(text: &str) -> Summary
```

Return line count and whitespace-separated word count.

In `src/format.rs`, define:

```rust
pub fn format_summary(summary: &crate::stats::Summary) -> String
```

Return:

```text
lines: {lines}, words: {words}
```

## Main Program

In `src/main.rs`:

1. Use the library crate by package name: `lib_modules`.
2. Create a string with two lines.
3. Call `lib_modules::stats::summarize`.
4. Call `lib_modules::format::format_summary`.
5. Print the result.

## Constraints

- Use `src/lib.rs`.
- Use separate `src/stats.rs` and `src/format.rs` files.
- Export modules with `pub mod`.
- Use `crate::stats::Summary` inside the library.
- Do not put all logic in `main.rs`.
- Do not use external crates.

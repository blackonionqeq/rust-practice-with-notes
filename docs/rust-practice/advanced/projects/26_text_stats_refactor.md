# 26 Text Stats Refactor

## Goal

Refactor the Stage 1 text statistics CLI into a small library plus binary, with tests and clearer APIs.

This is the Stage 2 integration project.

## Project

Create project:

```bash
cargo new practice/advanced/26_text_stats_refactor --name text_stats_refactor
```

## Required Files

Create:

```text
src/lib.rs
src/error.rs
src/stats.rs
src/words.rs
```

Keep:

```text
src/main.rs
```

## Required Modules

In `src/lib.rs`:

```rust
pub mod error;
pub mod stats;
pub mod words;
```

Re-export:

```rust
pub use error::AppError;
pub use stats::TextStats;
```

## Error Module

In `src/error.rs`, define:

```rust
#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    EmptyInput,
}
```

Implement:

- `std::fmt::Display`
- `From<std::io::Error>`

Required messages:

- `Io(_)`: `failed to read input`
- `EmptyInput`: `input is empty`

## Words Module

In `src/words.rs`, define:

```rust
pub fn normalize_word(word: &str) -> String
```

Trim leading and trailing non-alphanumeric characters and lowercase the word.

Define:

```rust
pub fn words(text: &str) -> Vec<String>
```

Split on whitespace, normalize words, and remove empty words.

Define:

```rust
pub fn count_words(words: &[String]) -> std::collections::HashMap<String, usize>
```

Return word frequencies.

## Stats Module

In `src/stats.rs`, define:

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub unique_words: usize,
}
```

Define:

```rust
pub fn stats(text: &str) -> Result<TextStats, crate::AppError>
```

Return `Err(crate::AppError::EmptyInput)` when `text.trim().is_empty()`.

Define:

```rust
pub fn top_words<F>(
    counts: &std::collections::HashMap<String, usize>,
    keep: F,
) -> Vec<(String, usize)>
where
    F: Fn(&str, usize) -> bool,
```

Return words that match `keep`, sorted by count descending, then word ascending.

## Main Program

In `src/main.rs`:

1. Read `sample.txt` with `std::fs::read_to_string`.
2. Use the library crate by package name: `text_stats_refactor`.
3. Print stats.
4. Print repeated words with count at least `2`.
5. Print `error: {error}` on failure.

## Required Tests

Add unit tests for:

- `normalize_word`
- `words`
- `count_words`
- `stats`
- `top_words`

## Constraints

- Use `lib.rs` plus modules.
- Keep file IO in `main.rs`; library functions should operate on `&str` or slices.
- Use `&[String]` instead of `&Vec<String>`.
- Use a closure in `top_words`.
- Use a custom `AppError`.
- Use `?` where appropriate.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use external crates.

## Review Focus

- Module boundaries.
- Public API signatures.
- Test coverage.
- Error handling.
- Ownership and cloning choices.

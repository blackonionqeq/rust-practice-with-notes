# 20 Text Stats CLI

## Goal

Build a small CLI-style text statistics program that combines collections, iterators, closures, error handling, and file IO.

This is the Stage 1 integration project.

## Project

Create project:

```bash
cargo new practice/advanced/20_text_stats_cli --name text_stats_cli
```

## Required Sample File

Create:

```text
sample.txt
```

with this content:

```text
Rust is fast
Rust is safe
Safe Rust code is practical
```

Put `sample.txt` in the project root, next to `Cargo.toml`.

## Required Struct

Define:

```rust
#[derive(Debug)]
struct TextStats {
    lines: usize,
    words: usize,
    unique_words: usize,
}
```

## Required Error Type

Define:

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    EmptyInput,
}
```

Implement:

- `std::fmt::Display` for `AppError`.
- `From<std::io::Error>` for `AppError`.

Required messages:

- `Io(_)`: `failed to read input`
- `EmptyInput`: `input is empty`

## Required Functions

```rust
fn read_text(path: &str) -> Result<String, AppError>
```

Read the file with `std::fs::read_to_string(path)?`.

```rust
fn normalize_word(word: &str) -> String
```

Return a lowercase version of the word with leading and trailing non-alphanumeric characters removed.

Examples:

- `"Rust"` -> `"rust"`
- `"safe,"` -> `"safe"`
- `"  Code!"` -> `"code"`

```rust
fn words(text: &str) -> Vec<String>
```

Split on whitespace, normalize each word, and remove empty words.

```rust
fn count_words(words: &[String]) -> std::collections::HashMap<String, usize>
```

Return word frequencies.

```rust
fn stats(text: &str) -> Result<TextStats, AppError>
```

Return `Err(AppError::EmptyInput)` when `text.trim().is_empty()`.

Otherwise return:

- line count
- word count
- unique word count

```rust
fn top_words<F>(
    counts: &std::collections::HashMap<String, usize>,
    keep: F,
) -> Vec<(String, usize)>
where
    F: Fn(&str, usize) -> bool,
```

Return words that match `keep`, sorted by count descending, then word ascending.

In `main`, use a closure to keep words whose count is at least `2`.

```rust
fn run(path: &str) -> Result<(), AppError>
```

Read the text, calculate stats, calculate top repeated words, and print:

```text
lines: {lines}
words: {words}
unique words: {unique_words}
repeated words:
{word}: {count}
```

## Main Program

In `main`:

1. Call `run("sample.txt")`.
2. Use `match`.
3. Print `error: {error}` on failure.

## Constraints

- Use `Vec<String>`.
- Use `HashMap<String, usize>`.
- Use iterator methods in `words`.
- Use a closure with `top_words`.
- Use a custom `AppError`.
- Use `?` for error propagation.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use external crates.
- Keep everything in `src/main.rs` for this project.

## Review Focus

- Correctness of word normalization.
- Ownership and borrowing in collection processing.
- Whether cloning is limited to places where owned keys or output values are needed.
- Clear `Result` flow.
- Formatting with `cargo fmt`.

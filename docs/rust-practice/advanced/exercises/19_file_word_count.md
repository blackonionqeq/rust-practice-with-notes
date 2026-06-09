# 19 File Word Count

## Goal

Practice file IO, `std::fs::read_to_string`, `Result`, and basic text processing.

## Project

Create project:

```bash
cargo new practice/advanced/19_file_word_count --name file_word_count
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
safe code matters
```

Put `sample.txt` in the project root, next to `Cargo.toml`.

## Required Function

```rust
fn read_text(path: &str) -> Result<String, std::io::Error>
```

Read the whole file with `std::fs::read_to_string(path)`.

## Required Function

```rust
fn count_words(text: &str) -> usize
```

Return the number of whitespace-separated words.

## Required Function

```rust
fn run(path: &str) -> Result<usize, std::io::Error>
```

Use `read_text(path)?`, then return `Ok(count_words(&text))`.

## Main Program

In `main`:

1. Call `run("sample.txt")`.
2. Use `match`.
3. Print `words: {count}` for success.
4. Print `error: {error}` for failure.

## Constraints

- Use `std::fs::read_to_string`.
- Use `?` inside `run`.
- Do not use `unwrap`.
- Do not use `expect`.
- Do not use external crates.
- Do not hardcode the word count.

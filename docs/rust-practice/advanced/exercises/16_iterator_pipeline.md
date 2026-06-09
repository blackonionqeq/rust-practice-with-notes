# 16 Iterator Pipeline

## Goal

Practice iterator adapters, lazy evaluation, and `collect`.

## Project

Create project:

```bash
cargo new practice/advanced/16_iterator_pipeline --name iterator_pipeline
```

## Required Function

```rust
fn clean_words(words: &[&str]) -> Vec<String>
```

Return a new vector that:

- Trims whitespace from each word.
- Converts each word to lowercase.
- Keeps only words whose character count is at least `3`.

Example input:

```rust
[" Rust ", "go", "LANG", "  js", "Ferris"]
```

Expected output:

```rust
["rust", "lang", "ferris"]
```

## Required Function

```rust
fn total_chars(words: &[String]) -> usize
```

Return the total character count of all words.

## Main Program

In `main`:

1. Create the sample input above.
2. Call `clean_words`.
3. Print the cleaned vector with `Debug`.
4. Call `total_chars`.
5. Print `total chars: {total}`.

## Constraints

- Use iterator methods in both required functions.
- Use `.iter()`, `.map()`, `.filter()`, and `.collect()` in `clean_words`.
- Use `.sum()` in `total_chars`.
- Count characters with `.chars().count()`, not `.len()`.
- Do not use a manual `for` loop in required functions.
- Do not use `clone`.

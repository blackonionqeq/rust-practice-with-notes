# 24 Unit Tests

## Goal

Practice unit tests, `#[cfg(test)]`, and assertions.

## Project

Create project:

```bash
cargo new practice/advanced/24_unit_tests --name unit_tests
```

## Required Library

Create `src/lib.rs`.

Define:

```rust
pub fn normalize_word(word: &str) -> String
```

Rules:

- Trim leading and trailing non-alphanumeric characters.
- Convert to lowercase.

Examples:

- `"Rust"` -> `"rust"`
- `"safe,"` -> `"safe"`
- `"  Code!"` -> `"code"`

Define:

```rust
pub fn count_words(text: &str) -> usize
```

Count whitespace-separated non-empty words after normalization.

## Required Tests

In `src/lib.rs`, add a test module:

```rust
#[cfg(test)]
mod tests {
    // tests here
}
```

Write at least 4 tests:

1. Normalizes simple uppercase text.
2. Trims punctuation.
3. Counts words across multiple lines.
4. Returns `0` for empty or whitespace-only input.

## Main Program

`src/main.rs` can be minimal:

```rust
fn main() {
    println!("run cargo test");
}
```

## Constraints

- Put test code under `#[cfg(test)]`.
- Use `assert_eq!`.
- Use library functions from tests.
- Run with `cargo test`.
- Do not use external crates.

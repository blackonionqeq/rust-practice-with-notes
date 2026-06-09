# 03 String Lab

## Goal

Practice `&str`, UTF-8 string length, character iteration, and collecting chars into `String`.

## Required Function

```rust
fn summarize(text: &str) -> (usize, usize, String)
```

## Review Result

Passed.

Output:

```text
bytes: 16
chars: 8
preview: Rus
```

## Notes

- `text.len()` returns bytes.
- `text.chars().count()` returns character count.
- `text.chars().take(3).collect()` is preferred over byte slicing for UTF-8 text.

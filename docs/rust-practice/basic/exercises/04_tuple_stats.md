# 04 Tuple Stats

## Goal

Practice tuples, ownership return, and string analysis.

## Project

Create project:

```bash
cargo new practice/basic/04_tuple_stats --name tuple_stats
```

## Required Function

```rust
fn analyze(text: String) -> (String, usize, usize)
```

Return:

- original string
- byte length
- character count

## Main Program

```rust
let text = String::from("Rust语言练习");
let (text, bytes, chars) = analyze(text);

println!("text: {}", text);
println!("bytes: {}", bytes);
println!("chars: {}", chars);
```

## Constraints

- `analyze` must receive `String`, not `&str`.
- Do not use `clone`.
- Return ownership to the caller through the tuple.

## Review Result

Passed.

Output:

```text
Text: Rust语言练习
bytes: 16
chars: 8
```

Notes:

- Computing `len()` and `chars().count()` before returning the tuple is the correct approach.
- `(text, text.len(), text.chars().count())` fails because tuple elements are evaluated left to right: moving `text` into the first element makes it unavailable for later borrowed reads.

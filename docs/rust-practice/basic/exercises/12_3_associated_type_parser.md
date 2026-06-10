# 12.3 Associated Type Parser

## Goal

Practice a trait with an associated type.

Associated types let a trait say:

> each implementor chooses one concrete output type for this trait.

## Project

Create project:

```bash
cargo new practice/basic/12_3_associated_type_parser --name associated_type_parser
```

## Required Trait

Define:

```rust
trait Parser {
    type Output;

    fn parse(&self, input: &str) -> Self::Output;
}
```

## Required Structs

Define:

```rust
struct NumberParser;
```

Define:

```rust
struct WordCountParser;
```

## Required Implementations

Implement `Parser` for `NumberParser`:

- `type Output = Option<i32>;`
- `parse` returns `input.parse::<i32>().ok()`.

Implement `Parser` for `WordCountParser`:

- `type Output = usize;`
- `parse` returns the number of whitespace-separated words.

## Main Program

In `main`:

1. Create a `NumberParser`.
2. Print the result of parsing `"42"`.
3. Print the result of parsing `"abc"`.
4. Create a `WordCountParser`.
5. Print the result of parsing `"rust is fun"`.

## Constraints

- Use one trait named `Parser`.
- Use one associated type named `Output`.
- Use `Self::Output` in the trait method return type.
- Do not use generics on the `Parser` trait.
- Do not use trait objects like `dyn Parser`.
- Do not use `unwrap` or `expect`.

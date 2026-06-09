# 11 Trait Summary

## Goal

Practice defining a trait and implementing it for structs.

## Project

Create project:

```bash
cargo new practice/basic/11_trait_summary --name trait_summary
```

## Required Trait

Define:

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

## Required Structs

Define:

```rust
struct Article {
    title: String,
    author: String,
}
```

Define:

```rust
struct Comment {
    username: String,
    content: String,
}
```

## Required Implementations

Implement `Summary` for `Article`:

- Return `{title} by {author}`.

Implement `Summary` for `Comment`:

- Return `{username}: {content}`.

## Main Program

In `main`:

1. Create an `Article` with title `Rust Notes` and author `black`.
2. Create a `Comment` with username `reader` and content `nice post`.
3. Print `article.summarize()`.
4. Print `comment.summarize()`.

## Constraints

- Use one `trait Summary`.
- Use two `impl Summary for ...` blocks.
- Use `&self` in `summarize`.
- Do not use generics.
- Do not use trait objects like `dyn Summary`.
- Do not use `clone`.

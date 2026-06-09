# 28 Rc Shared Tags

## Goal

Practice `Rc<T>` for shared ownership in single-threaded code.

## Project

Create project:

```bash
cargo new practice/advanced/28_rc_shared_tags --name rc_shared_tags
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Article {
    title: String,
    tags: Vec<std::rc::Rc<String>>,
}
```

## Required Functions

```rust
fn make_tag(name: &str) -> std::rc::Rc<String>
```

Return a new shared tag.

```rust
fn make_article(title: &str, tags: Vec<std::rc::Rc<String>>) -> Article
```

Return an article.

```rust
fn tag_names(article: &Article) -> Vec<&str>
```

Return tag names as borrowed string slices.

## Main Program

In `main`:

1. Create shared tags `rust` and `practice`.
2. Create two articles that both use the `rust` tag.
3. Print both articles with `Debug`.
4. Print `Rc::strong_count(&rust)` before and after creating articles.
5. Print tag names for one article.

## Constraints

- Use `Rc<String>` for shared tags.
- Use `Rc::clone(&tag)` when sharing ownership.
- Do not clone the inner `String`.
- Do not use `Arc`.
- Do not use `RefCell`.
- This exercise is single-threaded.

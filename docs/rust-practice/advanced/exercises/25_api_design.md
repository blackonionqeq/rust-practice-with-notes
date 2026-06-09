# 25 API Design

## Goal

Practice choosing parameter and return types that express ownership and borrowing clearly.

## Project

Create project:

```bash
cargo new practice/advanced/25_api_design --name api_design
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Report {
    title: String,
    tags: Vec<String>,
}
```

## Required Functions

```rust
fn make_report(title: impl Into<String>) -> Report
```

Create a report with the given title and an empty tag list.

```rust
fn add_tag(report: &mut Report, tag: impl Into<String>)
```

Add a tag.

```rust
fn has_tag(report: &Report, tag: &str) -> bool
```

Return whether the report contains the tag.

```rust
fn tags(report: &Report) -> &[String]
```

Return a read-only slice of tags.

```rust
fn into_title(report: Report) -> String
```

Consume the report and return its title.

## Main Program

In `main`:

1. Create a report with `make_report("Rust Review")`.
2. Add tags `rust` and `practice`.
3. Print whether the report has tag `rust`.
4. Print all tags through `tags(&report)`.
5. Call `into_title(report)` and print the title.

## Constraints

- Use `impl Into<String>` where specified.
- Use `&mut Report` for mutation.
- Use `&Report` for read-only access.
- Return `&[String]` from `tags`, not `&Vec<String>`.
- `into_title` must consume the report.
- Do not use `clone`.

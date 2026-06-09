# 22 Trait Objects

## Goal

Practice trait objects with `Box<dyn Trait>` and dynamic dispatch.

## Project

Create project:

```bash
cargo new practice/advanced/22_trait_objects --name trait_objects
```

## Required Trait

Define:

```rust
trait Render {
    fn render(&self) -> String;
}
```

## Required Structs

Define:

```rust
struct Text {
    value: String,
}
```

Define:

```rust
struct Button {
    label: String,
}
```

## Required Implementations

Implement `Render` for `Text`:

- Return `text: {value}`.

Implement `Render` for `Button`:

- Return `[button: {label}]`.

## Required Function

```rust
fn render_all(items: &[Box<dyn Render>]) -> Vec<String>
```

Return each rendered string.

## Main Program

In `main`:

1. Create a `Vec<Box<dyn Render>>`.
2. Push one `Text` and one `Button`.
3. Call `render_all(&items)`.
4. Print each rendered line.

## Constraints

- Use `Box<dyn Render>`.
- Use one vector containing both `Text` and `Button`.
- `render_all` must borrow the vector.
- Do not use generics for `render_all`.
- Do not use `enum` to combine the types.
- Do not use `clone`.

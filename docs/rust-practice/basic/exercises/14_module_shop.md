# 14 Module Shop

## Goal

Practice basic modules, `pub`, and paths with `::`.

## Project

Create project:

```bash
cargo new practice/basic/14_module_shop --name module_shop
```

## Module Structure

Write everything in `src/main.rs`.

Define a module:

```rust
mod shop {
    // code here
}
```

Inside `shop`, define a public nested module:

```rust
pub mod inventory {
    // code here
}
```

Inside `inventory`, define:

```rust
pub fn item_count() -> u32
```

Return `3`.

Inside `inventory`, define:

```rust
pub fn print_item(name: &str)
```

Print `item: {name}`.

## Required Function

Outside the modules, define:

```rust
fn print_shop_status()
```

In this function:

- Call `shop::inventory::item_count()`.
- Print `count: {count}`.
- Call `shop::inventory::print_item("book")`.

## Main Program

In `main`:

1. Call `print_shop_status()`.
2. Call `shop::inventory::print_item("pen")`.

## Constraints

- Use `mod shop`.
- Use nested `pub mod inventory`.
- Use `pub fn` for functions that are called from outside their module.
- Use paths with `::`.
- Keep everything in `src/main.rs`.
- Do not create extra files yet.
- Do not use `use` imports yet.

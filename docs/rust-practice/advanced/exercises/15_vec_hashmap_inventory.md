# 15 Vec HashMap Inventory

## Goal

Practice `Vec`, `HashMap`, ownership of collection elements, and basic counting.

## Project

Create project:

```bash
cargo new practice/advanced/15_vec_hashmap_inventory --name vec_hashmap_inventory
```

## Required Functions

```rust
fn build_items() -> Vec<String>
```

Return a `Vec<String>` containing:

- `book`
- `pen`
- `book`
- `notebook`
- `pen`
- `book`

```rust
fn count_items(items: &[String]) -> std::collections::HashMap<String, u32>
```

Return a map from item name to count.

Expected counts:

- `book`: `3`
- `pen`: `2`
- `notebook`: `1`

```rust
fn print_counts(counts: &std::collections::HashMap<String, u32>)
```

Print one line per item in this format:

```text
{item}: {count}
```

The print order does not matter.

## Main Program

In `main`:

1. Call `build_items`.
2. Call `count_items(&items)`.
3. Call `print_counts(&counts)`.
4. Print `total kinds: {len}` using `counts.len()`.

## Constraints

- Use `Vec<String>`.
- Use `HashMap<String, u32>`.
- `count_items` must borrow the vector as `&[String]`.
- Do not consume `items` inside `count_items`.
- Do not use `clone` in `build_items`.
- Do not use iterator chains yet; use a plain `for` loop.
- It is OK to clone item names when inserting keys into the map.

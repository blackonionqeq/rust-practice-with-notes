# 27 Box Recursive List

## Goal

Practice `Box<T>` and recursive data structures.

## Project

Create project:

```bash
cargo new practice/advanced/27_box_recursive_list --name box_recursive_list
```

## Required Enum

Define:

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

## Required Functions

```rust
fn prepend(list: List, value: i32) -> List
```

Return a new list with `value` at the front.

```rust
fn len(list: &List) -> usize
```

Return the number of values in the list.

```rust
fn sum(list: &List) -> i32
```

Return the sum of all values.

```rust
fn to_vec(list: &List) -> Vec<i32>
```

Return values from head to tail.

## Main Program

In `main`:

1. Start with `List::Nil`.
2. Prepend `3`, then `2`, then `1`.
3. Print the list with `Debug`.
4. Print `len: {len}`.
5. Print `sum: {sum}`.
6. Print the vector from `to_vec`.

## Constraints

- Use `Box<List>` in the enum.
- Do not use `Vec` as the internal list representation.
- `len`, `sum`, and `to_vec` must borrow the list.
- Do not use `clone`.
- Do not use `Rc`.

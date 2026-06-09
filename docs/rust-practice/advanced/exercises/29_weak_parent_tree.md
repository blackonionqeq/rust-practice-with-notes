# 29 Weak Parent Tree

## Goal

Practice `Rc<T>`, `Weak<T>`, and avoiding reference cycles.

## Project

Create project:

```bash
cargo new practice/advanced/29_weak_parent_tree --name weak_parent_tree
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Node {
    name: String,
    parent: std::cell::RefCell<std::rc::Weak<Node>>,
    children: std::cell::RefCell<Vec<std::rc::Rc<Node>>>,
}
```

## Required Functions

```rust
fn new_node(name: &str) -> std::rc::Rc<Node>
```

Create a node with no parent and no children.

```rust
fn add_child(parent: &std::rc::Rc<Node>, child: &std::rc::Rc<Node>)
```

Add `child` to `parent.children` and set `child.parent` to `Rc::downgrade(parent)`.

```rust
fn parent_name(node: &std::rc::Rc<Node>) -> Option<String>
```

Return the parent name when the weak reference can be upgraded.

## Main Program

In `main`:

1. Create `root`.
2. Create `leaf`.
3. Add `leaf` as a child of `root`.
4. Print `parent_name(&leaf)`.
5. Print `Rc::strong_count(&root)`.
6. Print `Rc::weak_count(&root)`.

## Constraints

- Use `Rc<Node>` for child ownership.
- Use `Weak<Node>` for parent reference.
- Use `RefCell` only for `parent` and `children`.
- Do not use `Arc`.
- Do not create a strong reference cycle.
- Do not use `unsafe`.

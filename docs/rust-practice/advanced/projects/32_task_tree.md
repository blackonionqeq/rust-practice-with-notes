# 32 Task Tree

## Goal

Build a small in-memory task tree that combines `Rc`, `Weak`, and `RefCell`.

This is the Stage 3 integration project.

## Project

Create project:

```bash
cargo new practice/advanced/32_task_tree --name task_tree
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Task {
    title: String,
    done: std::cell::RefCell<bool>,
    parent: std::cell::RefCell<std::rc::Weak<Task>>,
    children: std::cell::RefCell<Vec<std::rc::Rc<Task>>>,
}
```

## Required Functions

```rust
fn new_task(title: &str) -> std::rc::Rc<Task>
```

Create a task with no parent, no children, and `done == false`.

```rust
fn add_child(parent: &std::rc::Rc<Task>, child: &std::rc::Rc<Task>)
```

Attach `child` to `parent` and set the child's weak parent reference.

```rust
fn mark_done(task: &std::rc::Rc<Task>)
```

Set `done` to `true`.

```rust
fn is_done(task: &std::rc::Rc<Task>) -> bool
```

Return whether the task is done.

```rust
fn parent_title(task: &std::rc::Rc<Task>) -> Option<String>
```

Return the parent title when it exists.

```rust
fn child_titles(task: &std::rc::Rc<Task>) -> Vec<String>
```

Return child titles.

```rust
fn count_tasks(task: &std::rc::Rc<Task>) -> usize
```

Return the number of tasks in the subtree, including `task`.

## Main Program

In `main`:

1. Create a root task `Learn Rust`.
2. Create child tasks `Read notes`, `Do exercises`, and `Review code`.
3. Attach children to root.
4. Mark `Do exercises` as done.
5. Print root child titles.
6. Print parent title for `Do exercises`.
7. Print whether `Do exercises` is done.
8. Print total task count.
9. Print `Rc::strong_count(&root)` and `Rc::weak_count(&root)`.

## Constraints

- Use `Rc<Task>` for owning child references.
- Use `Weak<Task>` for parent references.
- Use `RefCell` for fields that need interior mutability.
- Do not use `Arc`.
- Do not use `Mutex`.
- Do not use `unsafe`.
- Do not create strong reference cycles.

## Review Focus

- Whether parent links use `Weak`.
- Whether interior mutability is limited to fields that need mutation through shared references.
- Ownership and borrowing in tree traversal.
- Avoiding unnecessary cloning.

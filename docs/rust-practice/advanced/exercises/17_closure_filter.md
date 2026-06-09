# 17 Closure Filter

## Goal

Practice closures, captured variables, and passing closures to functions.

## Project

Create project:

```bash
cargo new practice/advanced/17_closure_filter --name closure_filter
```

## Required Struct

Define:

```rust
#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
    priority: u8,
}
```

## Required Function

```rust
fn filter_tasks<F>(tasks: &[Task], predicate: F) -> Vec<&Task>
where
    F: Fn(&Task) -> bool,
```

Return references to tasks that match the predicate.

## Main Program

In `main`:

1. Create a `Vec<Task>` with at least 4 tasks.
2. Create a variable:

```rust
let min_priority = 3;
```

3. Use `filter_tasks` with a closure that keeps unfinished tasks whose priority is at least `min_priority`.
4. Print the filtered tasks with `Debug`.
5. Use `filter_tasks` again with a closure that keeps completed tasks.
6. Print the completed tasks with `Debug`.

## Constraints

- `filter_tasks` must return references, not cloned tasks.
- Use a generic closure parameter `F`.
- Use `Fn(&Task) -> bool` as the trait bound.
- The closure must capture `min_priority`.
- Do not use `dyn Fn`.
- Do not use `clone`.

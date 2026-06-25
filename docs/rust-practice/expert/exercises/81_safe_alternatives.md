# 81 Safe Alternatives to Self-Reference

## Goal

Compare three practical approaches to "a node that needs to refer back to related data", and understand when each is appropriate.

## Project

Create project:

```bash
cargo new practice/expert/81_safe_alternatives --name safe_alternatives
```

## Requirements

Use this toy scenario: a list of tasks where each task optionally points to a "parent" task.

### Approach 1: Index-based

```rust
struct Task {
    name: String,
    parent: Option<usize>,  // index into a Vec<Task>
}
```

Build a small `Vec<Task>` with a parent-child relationship. Write a function that walks up the parent chain and prints names.

### Approach 2: `Rc`-based

```rust
use std::rc::Rc;

struct TaskNode {
    name: String,
    parent: Option<Rc<TaskNode>>,
}
```

Build the same parent-child relationship using `Rc`. Print the parent's name from a child node.

### Approach 3: Comment on Pin

Do not write code for this approach. Add a comment block (5-8 lines) explaining:
- What problem `Pin` solves (stable memory address for self-referential raw pointers).
- Why you would only reach for it in very specific low-level situations (async runtimes, custom allocators).
- Why index-based or `Rc` covers the vast majority of real use cases.

In `main`, demonstrate both Approach 1 and Approach 2.

## Learning Scope

Already learned tools:

- `Vec`, `Option`, `Rc`
- `Pin` concept (79, 80)
- index-based workaround (77)

New concept in this exercise:

- practical trade-off analysis: when to use index, Rc, or Pin for back-references

Out of scope (do not use yet):

- `Weak<T>` for cycle breaking (related but separate topic)
- `Arc` (use `Rc` for single-threaded here)
- actual `Pin`-based self-reference implementation

## Constraints

- Keep code in `src/main.rs`.
- No extra crates.
- Approach 3 is comment-only — do not write `Pin` code.

## Review Focus

- Both Approach 1 and 2 compile and produce correct output.
- Comment for Approach 3 is concrete, not vague.
- Trade-offs are stated clearly: index = simple but indirect; Rc = shared ownership but heap cost; Pin = stable address but complex.

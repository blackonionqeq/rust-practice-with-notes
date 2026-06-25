# 74 Shrink Unsafe Scope

## Goal

Minimize the size of an unsafe block by moving safe code out of it.

## Project

Create project:

```bash
cargo new practice/expert/74_shrink_unsafe_scope --name shrink_unsafe_scope
```

## Requirements

Start with this function in `src/main.rs`:

```rust
fn sum_pair(data: &[i32], i: usize, j: usize) -> Option<i32> {
    unsafe {
        if i >= data.len() || j >= data.len() {
            return None;
        }
        let p = data.as_ptr();
        let a = *p.add(i);
        let b = *p.add(j);
        Some(a + b)
    }
}
```

Refactor it so that:

1. The bounds checks are outside the `unsafe` block.
2. `data.as_ptr()` is outside the `unsafe` block.
3. The `unsafe` block contains only the two pointer dereferences.
4. A `// Safety:` comment appears before the `unsafe` block.

In `main`:

1. Call `sum_pair` for one valid case (both indices in bounds) and print the result.
2. Call `sum_pair` where one index is out of bounds and print the result.

## Learning Scope

Already learned tools:

- safe wrapper pattern (72, 73)
- unsafe scope and raw pointer dereference

New concept in this exercise:

- the principle: unsafe blocks should be as small as possible

Out of scope (do not use yet):

- eliminating raw pointers entirely (for example by using `data.get(i)`)

## Constraints

- Keep code in `src/main.rs`.
- Final `unsafe` block must contain only the two pointer dereferences and no control flow.
- No extra crates.

## Review Focus

- `unsafe` block contains no branches and no safe-only operations.
- `// Safety:` comment names both index invariants.
- Code compiles and produces the same output as the original.

# 68 Unsafe Capabilities

## Goal

Recognize the five unsafe capabilities Rust permits by reading and annotating code examples.

## Project

Create project:

```bash
cargo new practice/expert/68_unsafe_capabilities --name unsafe_capabilities
```

## Background

Rust's `unsafe` keyword unlocks exactly five capabilities:

1. Dereference a raw pointer.
2. Call an unsafe function or method.
3. Access or modify a mutable static variable (`static mut`).
4. Implement an unsafe trait.
5. Access fields of a `union`.

## Requirements

Copy the following five snippets into `src/main.rs`:

```rust
// --- Snippet A ---
static mut COUNTER: u32 = 0;
fn increment() {
    unsafe {
        COUNTER += 1; // capability: ?
    }
}

// --- Snippet B ---
fn read_through_ptr() {
    let x: i32 = 42;
    let p: *const i32 = &x;
    unsafe {
        println!("{}", *p); // capability: ?
    }
}

// --- Snippet C ---
unsafe fn double(x: i32) -> i32 { x * 2 }
fn call_unsafe() {
    unsafe {
        double(5); // capability: ?
    }
}

// --- Snippet D ---
union Bits { i: i32, b: [u8; 4] }
fn read_union() {
    let v = Bits { i: 0x01020304 };
    unsafe {
        println!("{}", v.i); // capability: ?
    }
}

// --- Snippet E ---
unsafe trait Trusted {}
unsafe impl Trusted for i32 {} // capability: ?
```

For each `// capability: ?` comment, replace `?` with the name of the capability being used.

Then add one new function:

```rust
fn my_deref_example() {
    // Write your own dereference of a raw pointer here.
    // Derive the pointer from a valid reference, not a raw address.
    // Print the value.
}
```

Call `increment`, `read_through_ptr`, `call_unsafe`, `read_union`, and `my_deref_example` from `main`.

## Learning Scope

Already learned tools:

- references and borrowing
- functions and basic types

New knowledge points in this exercise:

- the five unsafe capabilities and how to identify them
- difference between allowing an unsafe capability and disabling Rust checks
- recognizing where unsafe responsibility sits

Out of scope (do not use yet):

- writing safe wrappers
- raw pointer arithmetic
- safety contract comments

## Constraints

- Keep code in `src/main.rs`.
- Do not add new unsafe blocks beyond those in the snippets and `my_deref_example`.
- `my_deref_example` must derive its pointer from a stack variable (not a raw integer address).

## Review Focus

- All five capabilities are correctly labeled.
- Own example is minimal and derives the pointer from a valid reference.

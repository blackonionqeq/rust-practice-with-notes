# Rust Practice

This directory tracks the Rust review sequence.

## Source Notes

The user's Rust notes are exported under:

- `rust-notes/black的笔记.html`

The review plan follows those notes from oldest to newest. Each Rust concept gets one focused exercise.

## Sections

- Basic: `docs/rust-practice/basic/README.md`
- Advanced: `docs/rust-practice/advanced/README.md`
- Advanced Extra: `docs/rust-practice/advanced-extra/README.md`
- Expert: `docs/rust-practice/expert/README.md`

## Practice Projects

Runnable Cargo projects are under:

- Basic projects: `practice/basic/`
- Advanced projects: `practice/advanced/`
- Advanced Extra projects: `practice/advanced-extra/`
- Expert projects: `practice/expert/`

## Current Step

Current section:

- Advanced

Next exercise:

- `docs/rust-practice/advanced/exercises/15_vec_hashmap_inventory.md`

## Topic Order

1. Basic: command line / Cargo, ownership, strings, tuples, structs, enums, Option combinators, arrays, flow control, pattern matching, methods, traits, error handling, lifetimes, modules.
2. Advanced: collections, iterators, closures, practical error handling, file IO, generics, trait objects, project structure, tests, API design, smart pointers, shared mutability, and basic thread-safe shared state. A mid-term Rust + WebAssembly direction is tracked in `docs/rust-practice/advanced/guides/wasm_after_advanced.md`.
3. Advanced Extra: real CLI and IO, Cargo ecosystem, practical error crates, serde / JSON, integration tests, doc tests, workspaces, feature flags, message-passing concurrency, `RwLock`, and application-level async with Tokio.
4. Expert: `'static`, `Sized` / DST, global state, macros, unsafe, FFI, `Pin`, and self-reference.

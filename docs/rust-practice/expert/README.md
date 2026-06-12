# Rust Practice Expert

This directory tracks advanced boundary topics that are useful for understanding lower-level Rust, unsafe code, macros, FFI, and library internals.

Expert topics are not required for everyday Rust practice. The goal is to learn the boundaries carefully, not to use these tools casually.

## Status

Outline only. Exercises, notes, and projects will be filled in later.

## Relationship to Earlier Sections

- Basic: learn the language fundamentals.
- Advanced: learn core intermediate Rust mechanisms and small standard-library projects.
- Advanced Extra: practice real project skills, ecosystem crates, testing, concurrency, and application-level async.
- Expert: understand sharp edges, unsafe boundaries, and lower-level abstractions.

## Study Rules

1. Prefer safe Rust whenever possible.
2. Treat `unsafe` as a small implementation detail behind a safe API.
3. Write down the invariants that unsafe code relies on.
4. Keep unsafe blocks small.
5. Add focused tests around safe wrappers.
6. Do not use expert topics just to make code shorter.

## Planned Stages

### Stage 4: Lifetimes, Static Data, and Dynamically Sized Types

Goal: clarify advanced type and lifetime boundaries before touching unsafe-heavy code.

Planned topics:

1. `'static` vs `&'static T`
2. `Box::leak` and intentional leaks
3. `Sized`, `?Sized`, `str`, `[T]`, and `dyn Trait`
4. Type aliases and newtype wrappers
5. Practical API boundaries with DSTs

Planned project:

- A small registry-style program that uses owned strings, borrowed static strings, and safe global initialization.

### Stage 5: Global State and Initialization

Goal: understand when global data is reasonable and how to initialize it safely.

Planned topics:

1. `const`
2. `static`
3. `static mut` and why to avoid it
4. Atomic types
5. `OnceLock` / `LazyLock`

Planned project:

- A small configuration registry using safe one-time initialization.

### Stage 6: Macro Basics

Goal: understand macros enough to read common Rust code and write simple local helpers.

Planned topics:

1. `macro_rules!`
2. Repetition patterns
3. Hygiene basics
4. Derive macros at a conceptual level
5. When not to use a macro

Planned project:

- A small logging or assertion helper using `macro_rules!`.

### Stage 7: Unsafe Basics

Goal: learn what `unsafe` permits and how to wrap it in safe APIs.

Planned topics:

1. The five unsafe capabilities
2. Raw pointers
3. Dereferencing raw pointers
4. Unsafe functions
5. Safe wrappers around unsafe internals

Planned project:

- A small safe abstraction over a raw-pointer operation.

### Stage 8: FFI Basics

Goal: understand how Rust calls C-style APIs and how boundaries affect safety.

Planned topics:

1. `extern "C"`
2. `#[repr(C)]`
3. C string boundaries
4. Ownership across FFI
5. Error handling at FFI boundaries

Planned project:

- A minimal Rust-to-C style boundary exercise.

### Stage 9: Pin and Self-Reference

Goal: understand why self-referential data is hard and what `Pin` is trying to protect.

Planned topics:

1. Self-referential struct problem
2. Moving values and invalidating internal references
3. `Pin` conceptually
4. `Unpin`
5. When to use existing crates instead of hand-rolling

Planned project:

- A conceptual exercise comparing safe alternatives before using `Pin`.

## Deep Reference

The Rustonomicon is the main deep reference for unsafe Rust. Treat it as a specialist reference, not a required first pass.

Suggested use:

- Read only the chapter related to the current expert topic.
- Summarize invariants in your own words.
- Prefer writing safe examples before unsafe examples.

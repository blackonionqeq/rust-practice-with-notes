# Rust Practice Expert

This directory tracks advanced boundary topics that are useful for understanding lower-level Rust, unsafe code, macros, FFI, and library internals.

Expert topics are not required for everyday Rust practice. The goal is to learn the boundaries carefully, not to use these tools casually.

## Status

E1 exercises and notes are initialized (58-62). Later stages and projects are still outline-level.

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
7. Keep learning load low: each exercise should introduce at most one clearly new concept.
8. Do not require unfamiliar crates or advanced syntax unless the exercise explicitly teaches them.

## Exercise Design Constraint (Low Cognitive Load)

To avoid "learned helplessness" and keep momentum:

1. Every exercise must declare:
	- Already learned tools you may use directly
	- One new concept introduced in this exercise
	- Out-of-scope techniques (do not use yet)
2. Prefer "single-variable" tasks:
	- Change one dimension at a time (ownership, API shape, unsafe boundary, etc.)
3. Avoid hidden complexity:
	- No surprise macro magic, advanced trait machinery, or async internals in unrelated topics
4. Use incremental variants:
	- Base version (must pass) -> optional challenge version (nice to have)

## Planned Stages

## Initial Exercise Set (E1)

These are the first five concrete expert exercises, designed with low cognitive load.

1. `58_static_lifetime_basics`: `docs/rust-practice/expert/exercises/58_static_lifetime_basics.md`
2. `59_box_leak_intentional`: `docs/rust-practice/expert/exercises/59_box_leak_intentional.md`
3. `60_sized_qsized_signatures`: `docs/rust-practice/expert/exercises/60_sized_qsized_signatures.md`
4. `61_dst_api_boundaries`: `docs/rust-practice/expert/exercises/61_dst_api_boundaries.md`
5. `62_oncelock_config_basics`: `docs/rust-practice/expert/exercises/62_oncelock_config_basics.md`

Progression principle:

- each exercise introduces one new concept
- no surprise dependencies on unlearned advanced machinery
- optional challenge variants can be added later, but base task stays minimal

Notes style (from advanced-extra onward):

- concise by default, not long tutorial prose
- explicitly list prerequisite knowledge points used by this exercise
- focus on core takeaway, common pitfalls, and quick self-check questions

Supplement notes:

- `58_static_lifetime_basics`: `docs/rust-practice/expert/notes/58_static_lifetime_basics.md`
- `59_box_leak_intentional`: `docs/rust-practice/expert/notes/59_box_leak_intentional.md`
- `60_sized_qsized_signatures`: `docs/rust-practice/expert/notes/60_sized_qsized_signatures.md`
- `61_dst_api_boundaries`: `docs/rust-practice/expert/notes/61_dst_api_boundaries.md`
- `62_oncelock_config_basics`: `docs/rust-practice/expert/notes/62_oncelock_config_basics.md`

## Exercise Set (E2) - Macro Basics

These exercises map to Stage 6 and keep one-new-concept-per-exercise pacing.

1. `63_macro_rules_basic`: `docs/rust-practice/expert/exercises/63_macro_rules_basic.md`
2. `64_macro_repetition_join`: `docs/rust-practice/expert/exercises/64_macro_repetition_join.md`
3. `65_macro_hygiene_basics`: `docs/rust-practice/expert/exercises/65_macro_hygiene_basics.md`
4. `66_derive_macro_concept`: `docs/rust-practice/expert/exercises/66_derive_macro_concept.md`
5. `67_macro_vs_function`: `docs/rust-practice/expert/exercises/67_macro_vs_function.md`

Supplement notes:

- `63_macro_rules_basic`: `docs/rust-practice/expert/notes/63_macro_rules_basic.md`
- `64_macro_repetition_join`: `docs/rust-practice/expert/notes/64_macro_repetition_join.md`
- `65_macro_hygiene_basics`: `docs/rust-practice/expert/notes/65_macro_hygiene_basics.md`
- `66_derive_macro_concept`: `docs/rust-practice/expert/notes/66_derive_macro_concept.md`
- `67_macro_vs_function`: `docs/rust-practice/expert/notes/67_macro_vs_function.md`

## Mini Project Plan (Expert)

Expert should keep the same cadence as advanced-extra: every few exercises, build one small integration project.

Suggested projects:

1. `P1_registry_bootstrap` (after Stage 4 + Stage 5)
	- Scope: owned + borrowed config labels, one-time init, DST-friendly read-only API.
	- Keep small: single binary + one small `lib.rs` module.
	- Review focus: API boundaries (`&str`/`&[T]` style), no unnecessary ownership moves, no `static mut`.

2. `P2_macro_rules_toolkit` (after Stage 6)
	- Scope: 1-2 tiny `macro_rules!` helpers (e.g. assertion/log formatting wrapper).
	- Keep small: no proc-macro, no multi-crate setup.
	- Review focus: macro readability, hygiene basics, clear "when not to use macro" notes.

3. `P3_safe_unsafe_wrapper` (after Stage 7, main capstone)
	- Scope: one safe API wrapping a tiny unsafe core (raw pointer or memory view style).
	- Keep small: one core abstraction, explicit invariant list, focused unit tests.
	- Review focus: unsafe block size, safety contract clarity, tests that defend invariants.

4. `P4_ffi_pin_boundary_lab` (after Stage 8 + Stage 9)
	- Scope: small FFI boundary sample + small Pin/self-reference comparison demo.
	- Keep small: conceptual boundary exercise, avoid executor/future internals.
	- Review focus: ownership across boundaries, representation assumptions, when to prefer safe alternatives.

Project design rule:

- expert projects integrate boundary thinking, not feature volume
- each project should only combine nearby stages
- if complexity grows, split into base version and optional challenge

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

Pacing note: this stage should be longer than others and split into smaller, safer steps.

Planned topics:

1. The five unsafe capabilities
2. Raw pointers
3. Dereferencing raw pointers
4. Unsafe functions
5. Safe wrappers around unsafe internals

Recommended exercise rhythm for this stage:

1. Capability recognition (identify where unsafe is actually required)
2. Minimal raw pointer reads (single function, no abstraction)
3. Minimal raw pointer writes (single invariant)
4. Unsafe function boundary + explicit safety contract
5. Safe wrapper v1 (single invariant)
6. Safe wrapper v2 (multiple invariants + focused tests)
7. Boundary review exercise (find and shrink unnecessary unsafe)

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

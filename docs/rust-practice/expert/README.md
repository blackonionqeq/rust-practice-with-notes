# Rust Practice Expert

This directory tracks advanced boundary topics that are useful for understanding lower-level Rust, unsafe code, macros, FFI, and library internals.

Expert topics are not required for everyday Rust practice. The goal is to learn the boundaries carefully, not to use these tools casually.

## Status

- E1 (58-62): exercises and notes complete.
- E2 (63-67): exercises and notes complete.
- Stage 7 (68-74): exercises and notes complete.
- Stage 8 (75-76): exercises and notes complete.
- Stage 9 (77-81): exercises and notes complete.
- P1, P2, P3: project docs written, not yet built.
- P4: planned, not yet written.

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
7. Keep learning load low: each exercise should introduce a small cluster of 1-4 tightly related knowledge points, not several unrelated topics.
8. Do not require unfamiliar crates or advanced syntax unless the exercise explicitly teaches them.

## Exercise Design Constraint (Low Cognitive Load)

To avoid "learned helplessness" and keep momentum:

1. Every exercise must declare:
	- Already learned tools you may use directly
	- New knowledge points introduced in this exercise
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

- each exercise introduces 1-4 tightly related knowledge points
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

These exercises map to Stage 6 and keep a small-cluster-per-exercise pacing.

1. `63_macro_rules_basic`: `docs/rust-practice/expert/exercises/63_macro_rules_basic.md`
2. `64_macro_repetition_join`: `docs/rust-practice/expert/exercises/64_macro_repetition_join.md`
3. `65_macro_hygiene_basics`: `docs/rust-practice/expert/exercises/65_macro_hygiene_basics.md`
4. `66_macro_vs_function`: `docs/rust-practice/expert/exercises/66_macro_vs_function.md`
5. `67_derive_manual_impl`: `docs/rust-practice/expert/exercises/67_derive_manual_impl.md`

Progression rationale for E2:

- 63-65: learn to write macros (single arg → repetition → hygiene)
- 66: learn when NOT to use macros (function is often better)
- 67: understand derive macros by writing equivalent impls manually first

Supplement notes:

- `63_macro_rules_basic`: `docs/rust-practice/expert/notes/63_macro_rules_basic.md`
- `64_macro_repetition_join`: `docs/rust-practice/expert/notes/64_macro_repetition_join.md`
- `65_macro_hygiene_basics`: `docs/rust-practice/expert/notes/65_macro_hygiene_basics.md`
- `66_macro_vs_function`: `docs/rust-practice/expert/notes/66_macro_vs_function.md`
- `67_derive_manual_impl`: `docs/rust-practice/expert/notes/67_derive_manual_impl.md`

## Mini Project Plan (Expert)

Expert projects appear after each stage group to integrate the concepts before moving on.

### P1: `registry_bootstrap` (after E1, before E2)

- `docs/rust-practice/expert/projects/P1_registry_bootstrap.md`
- Integrates: `'static` refs, `Box::leak`, DST API boundaries, `OnceLock`.
- Scope: single binary, one small registry module in `main.rs`.

### P2: `macro_rules_toolkit` (after E2, before Stage 7)

- `docs/rust-practice/expert/projects/P2_macro_rules_toolkit.md`
- Integrates: `macro_rules!`, repetition, hygiene, macro vs function trade-off.
- Scope: two small helpers (`assert_contains!`, `log_fields!`) with tests.

### P3: `safe_unsafe_wrapper` (after Stage 7)

- `docs/rust-practice/expert/projects/P3_safe_unsafe_wrapper.md`
- Integrates: raw pointers, unsafe fn, safe wrapper with invariant docs and focused tests.
- Scope: `SliceIndex` struct with safe public API, `PhantomData` for lifetime, four unit tests.

### P4: `pin_self_ref_lab` (after Stage 9)

- Planned.
- Integrates: self-reference problem, Pin conceptually, safe alternatives.
- Scope: conceptual boundary exercise — no async executor internals.

Project design rule:

- expert projects integrate boundary thinking, not feature volume
- each project should only combine the immediately preceding stages
- if complexity grows, split into base version and optional challenge

### Stage 4 + 5: Lifetimes, Static Data, DSTs, and Global State

**Complete** — covered by E1 (exercises 58-62) and P1.

Topics covered: `'static` vs `T: 'static`, `Box::leak`, `?Sized`, DST API boundaries, `OnceLock`.

### Stage 6: Macro Basics

**Complete** — covered by E2 (exercises 63-67) and P2.

Topics covered: `macro_rules!`, repetition patterns, hygiene basics, macro vs function trade-off, derive macros via manual impl comparison.

### Stage 7: Unsafe Basics

Goal: learn what `unsafe` permits and how to wrap it in safe APIs.

Pacing note: this stage is longer than others by design. Unsafe requires recognizing invariants before writing code.

Exercises (68-74):

1. `68_unsafe_capabilities`: read code examples, annotate which of the five unsafe capabilities each block uses. No new code to write — recognition only.
2. `69_raw_pointer_read`: create a `*const T` from a reference and dereference it inside an unsafe block. Single function, no abstraction layer yet.
3. `70_raw_pointer_write`: create a `*mut T` from a mutable reference and write through it. State the one invariant the code relies on.
4. `71_unsafe_fn_contract`: define an `unsafe fn` with a `# Safety` doc comment. Show a correct and an incorrect call site.
5. `72_safe_wrapper_v1`: wrap a single raw pointer operation behind a safe public function. One invariant, enforced at the boundary.
6. `73_safe_wrapper_v2`: extend the wrapper with a second invariant and add at least two focused unit tests.
7. `74_shrink_unsafe_scope`: given a snippet with an oversized unsafe block, refactor it to minimize the unsafe surface.

Supplement notes:

- `68_unsafe_capabilities`: `docs/rust-practice/expert/notes/68_unsafe_capabilities.md`
- `69_raw_pointer_read`: `docs/rust-practice/expert/notes/69_raw_pointer_read.md`
- `70_raw_pointer_write`: `docs/rust-practice/expert/notes/70_raw_pointer_write.md`
- `71_unsafe_fn_contract`: `docs/rust-practice/expert/notes/71_unsafe_fn_contract.md`
- `72_safe_wrapper_v1`: `docs/rust-practice/expert/notes/72_safe_wrapper_v1.md`
- `73_safe_wrapper_v2`: `docs/rust-practice/expert/notes/73_safe_wrapper_v2.md`
- `74_shrink_unsafe_scope`: `docs/rust-practice/expert/notes/74_shrink_unsafe_scope.md`

Planned project:

- `P3_safe_unsafe_wrapper` (after exercise 74)

### Stage 8: FFI Basics (Lightweight)

Goal: understand how Rust communicates with C-style APIs at the type level. No actual C compilation required.

Scope note: this stage is intentionally short — two focused exercises, no build.rs, no linking. If you later need real FFI for a specific project, read the Rustonomicon FFI chapter at that point.

Exercises (75-76):

1. `75_repr_c_struct`: define a `#[repr(C)]` struct and compare its layout to a normal Rust struct. Use `std::mem::size_of` and `offset_of!` to inspect. Write an `extern "C"` function declaration (no implementation needed — just show the type boundary).
2. `76_cstr_cstring_boundary`: practice converting between `&CStr`, `CString`, and `&str`. Handle the null-terminator boundary safely without `unsafe`. Understand when the conversion can fail.

Supplement notes:

- `75_repr_c_struct`: `docs/rust-practice/expert/notes/75_repr_c_struct.md`
- `76_cstr_cstring_boundary`: `docs/rust-practice/expert/notes/76_cstr_cstring_boundary.md`

No project planned for Stage 8.

Note: full FFI (linking to actual C code, `build.rs`, `bindgen`) is out of scope for this curriculum.

### Stage 9: Pin and Self-Reference

Goal: understand why self-referential data is hard and what `Pin` is protecting, without diving into async executor internals.

Exercises (77-81):

1. `77_self_ref_problem`: build a struct that tries to hold a reference to its own field. Observe the compiler error. Understand why moving breaks the reference.
2. `78_move_invalidates_ref`: write a small example that shows how stack moves work and why they invalidate internal pointers. Use raw pointers temporarily to make the problem concrete.
3. `79_pin_concept`: use `Pin<Box<T>>` to create an unmovable value. Implement `!Unpin` with `PhantomPinned`. Verify that move is rejected at compile time.
4. `80_unpin_escape`: implement `Unpin` manually (or use a type that is `Unpin`) and show that `Pin` adds no extra restriction in that case.
5. `81_safe_alternatives`: compare three approaches to self-reference (index-based, `Rc<T>`, and a comment on Pin) for the same toy problem. Comment on trade-offs.

Supplement notes:

- `77_self_ref_problem`: `docs/rust-practice/expert/notes/77_self_ref_problem.md`
- `78_move_invalidates_ref`: `docs/rust-practice/expert/notes/78_move_invalidates_ref.md`
- `79_pin_concept`: `docs/rust-practice/expert/notes/79_pin_concept.md`
- `80_unpin_escape`: `docs/rust-practice/expert/notes/80_unpin_escape.md`
- `81_safe_alternatives`: `docs/rust-practice/expert/notes/81_safe_alternatives.md`

Planned project:

- `P4_pin_self_ref_lab` (after exercise 81)
  - Scope: compare safe alternatives and one `Pin`-based approach side by side.
  - No async executor internals.

## Deep Reference

The Rustonomicon is the main deep reference for unsafe Rust. Treat it as a specialist reference, not a required first pass.

Suggested use:

- Read only the chapter related to the current expert topic.
- Summarize invariants in your own words.
- Prefer writing safe examples before unsafe examples.

# Rust Practice Advanced

This directory tracks the advanced Rust review sequence.

The advanced section uses a mixed rhythm:

1. Learn 3-5 focused concepts.
2. Complete one small exercise for each concept.
3. Complete one small project that combines those concepts.

## Mid-Term Direction

After Advanced, one practical direction is Rust + WebAssembly:

- `docs/rust-practice/advanced/guides/wasm_after_advanced.md`

## Current Step

Next exercise:

- `docs/rust-practice/advanced/exercises/15_vec_hashmap_inventory.md`

## Stage 1: Practical Data Processing

Goal: move from isolated syntax practice to small practical programs that process data, transform collections, use closures, and return useful errors.

### Topics

1. `Vec` / `HashMap`
2. Iterators
3. Closures
4. Advanced error handling
5. File IO

### Exercises

- `15_vec_hashmap_inventory`: `docs/rust-practice/advanced/exercises/15_vec_hashmap_inventory.md`
- `16_iterator_pipeline`: `docs/rust-practice/advanced/exercises/16_iterator_pipeline.md`
- `17_closure_filter`: `docs/rust-practice/advanced/exercises/17_closure_filter.md`
- `18_error_from`: `docs/rust-practice/advanced/exercises/18_error_from.md`
- `19_file_word_count`: `docs/rust-practice/advanced/exercises/19_file_word_count.md`

### Supplement Notes

- `15_vec_hashmap_inventory`: `docs/rust-practice/advanced/notes/15_vec_hashmap_inventory.md`
- `16_iterator_pipeline`: `docs/rust-practice/advanced/notes/16_iterator_pipeline.md`
- `17_closure_filter`: `docs/rust-practice/advanced/notes/17_closure_filter.md`
- `18_error_from`: `docs/rust-practice/advanced/notes/18_error_from.md`
- `19_file_word_count`: `docs/rust-practice/advanced/notes/19_file_word_count.md`

### Project

- `20_text_stats_cli`: `docs/rust-practice/advanced/projects/20_text_stats_cli.md`

## Stage 2: Abstractions and Project Structure

Goal: turn small working programs into clearer Rust projects with reusable APIs, module boundaries, tests, and appropriate abstraction choices.

### Topics

1. Generics and trait bounds
2. Trait objects
3. `lib.rs` and multi-file modules
4. Unit tests
5. API signature design

### Exercises

- `21_generics_bounds`: `docs/rust-practice/advanced/exercises/21_generics_bounds.md`
- `22_trait_objects`: `docs/rust-practice/advanced/exercises/22_trait_objects.md`
- `23_lib_modules`: `docs/rust-practice/advanced/exercises/23_lib_modules.md`
- `24_unit_tests`: `docs/rust-practice/advanced/exercises/24_unit_tests.md`
- `25_api_design`: `docs/rust-practice/advanced/exercises/25_api_design.md`

### Supplement Notes

- `21_generics_bounds`: `docs/rust-practice/advanced/notes/21_generics_bounds.md`
- `22_trait_objects`: `docs/rust-practice/advanced/notes/22_trait_objects.md`
- `23_lib_modules`: `docs/rust-practice/advanced/notes/23_lib_modules.md`
- `24_unit_tests`: `docs/rust-practice/advanced/notes/24_unit_tests.md`
- `25_api_design`: `docs/rust-practice/advanced/notes/25_api_design.md`

### Project

- `26_text_stats_refactor`: `docs/rust-practice/advanced/projects/26_text_stats_refactor.md`

## Stage 3: Smart Pointers and Shared Mutability

Goal: understand Rust's common ownership tools for recursive structures, shared ownership, parent-child graphs, interior mutability, and basic thread-safe shared state.

### Topics

1. `Box<T>` and recursive data
2. `Rc<T>` shared ownership
3. `Weak<T>` parent references
4. `RefCell<T>` interior mutability
5. `Arc<T>` and `Mutex<T>` for threads

### Exercises

- `27_box_recursive_list`: `docs/rust-practice/advanced/exercises/27_box_recursive_list.md`
- `28_rc_shared_tags`: `docs/rust-practice/advanced/exercises/28_rc_shared_tags.md`
- `29_weak_parent_tree`: `docs/rust-practice/advanced/exercises/29_weak_parent_tree.md`
- `30_refcell_counter`: `docs/rust-practice/advanced/exercises/30_refcell_counter.md`
- `31_arc_mutex_threads`: `docs/rust-practice/advanced/exercises/31_arc_mutex_threads.md`

### Supplement Notes

- `27_box_recursive_list`: `docs/rust-practice/advanced/notes/27_box_recursive_list.md`
- `28_rc_shared_tags`: `docs/rust-practice/advanced/notes/28_rc_shared_tags.md`
- `29_weak_parent_tree`: `docs/rust-practice/advanced/notes/29_weak_parent_tree.md`
- `30_refcell_counter`: `docs/rust-practice/advanced/notes/30_refcell_counter.md`
- `31_arc_mutex_threads`: `docs/rust-practice/advanced/notes/31_arc_mutex_threads.md`

### Project

- `32_task_tree`: `docs/rust-practice/advanced/projects/32_task_tree.md`

## Next Section

Advanced is followed by Advanced Extra:

- `docs/rust-practice/advanced-extra/README.md`

Advanced Extra topics include:

- real CLI and streaming IO
- practical Cargo ecosystem usage
- `thiserror` / `anyhow`
- `serde` / JSON config
- integration tests and doc tests
- workspaces and feature flags
- message-passing concurrency
- application-level async with Tokio

After Advanced Extra, continue to Expert:

- `docs/rust-practice/expert/README.md`

Expert topics include:

- `'static`
- `Sized` / DST
- global state
- macros
- unsafe
- FFI
- `Pin`
- self-reference

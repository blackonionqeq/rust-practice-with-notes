# AGENTS.md

## Context

This repository is used for Rust review through small practice projects.

Read the exercise index and current next step from:

- `docs/rust-practice/README.md`

Each exercise has its own file under:

- `docs/rust-practice/basic/exercises/`
- `docs/rust-practice/advanced/exercises/`
- `docs/rust-practice/advanced-extra/exercises/`
- `docs/rust-practice/expert/exercises/`

Stage projects are under:

- `docs/rust-practice/advanced/projects/`
- `docs/rust-practice/advanced-extra/projects/`
- `docs/rust-practice/expert/projects/`

Runnable Cargo practice projects are under:

- `practice/basic/`
- `practice/advanced/`
- `practice/advanced-extra/`
- `practice/expert/`

The user's notes are exported under:

- `rust-notes/black的笔记.html`

The review plan follows those notes from oldest to newest. Each exercise should stay focused, but may cover 1-4 tightly related knowledge points when one idea needs nearby context to be understandable. After the user finishes an exercise and says `done`, inspect the relevant project, run it, run `cargo fmt --check`, then give a short review and the next exercise.

## Interaction Style

- Communicate in Chinese.
- Keep each exercise small and practical.
- Review like a code reviewer: correctness first, then Rust idioms, ownership/borrowing, formatting, and next steps.
- Do not over-explain unless the user asks.
- When assigning the next exercise, include requirements and constraints.

## Project Naming

Practice directories use numeric prefixes for review order:

- `01_display_alpha`
- `02_try_shadowing`
- `03_string_lab`
- `15_vec_hashmap_inventory`
- `33_cli_args_pathbuf`
- next exercise is tracked in `docs/rust-practice/README.md`

Cargo package names must not start with digits. Use a valid package name in `Cargo.toml`, for example:

```toml
[package]
name = "string_lab"
```

For new projects, prefer:

```bash
cargo new practice/basic/04_tuple_stats --name tuple_stats
```

For advanced projects, continue the numeric sequence:

```bash
cargo new practice/advanced/15_vec_hashmap_inventory --name vec_hashmap_inventory
```

For advanced-extra projects, continue the numeric sequence after advanced:

```bash
cargo new practice/advanced-extra/33_cli_args_pathbuf --name cli_args_pathbuf
```

For expert projects, use the expert directory:

```bash
cargo new practice/expert/58_static_dst --name static_dst
```

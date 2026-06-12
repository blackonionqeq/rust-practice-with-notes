# 50 RwLock Read Mostly

## Goal

Practice `Arc<RwLock<T>>` for read-heavy shared state.

## Project

Create project:

```bash
cargo new practice/advanced-extra/50_rwlock_read_mostly --name rwlock_read_mostly
```

## Requirements

- Create `Arc<RwLock<HashMap<String, usize>>>`.
- Spawn reader threads that look up values.
- Spawn one writer thread that inserts or updates a value.
- Print reader results and final map size.

## Constraints

- Use `RwLock` instead of `Mutex`.
- Keep lock guards scoped as narrowly as possible.
- Handle poisoned locks without `unwrap` or `expect`.
- Do not hold a write lock while doing unrelated work.

## Review Focus

- Appropriate read/write locking.
- Short lock scopes.
- Understanding poisoned lock handling.

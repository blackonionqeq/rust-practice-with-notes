# 51 Lock Scope and Deadlock

## Goal

Practice recognizing and avoiding lock scope problems.

## Project

Create project:

```bash
cargo new practice/advanced-extra/51_lock_scope_deadlock --name lock_scope_deadlock
```

## Requirements

- Create two shared counters protected by `Mutex`.
- Write one safe function that updates both counters in a consistent lock order.
- Add comments explaining why inconsistent lock ordering can deadlock.
- Demonstrate that the safe function works with multiple threads.

## Constraints

- Do not intentionally leave the program deadlocking.
- Use a consistent lock acquisition order.
- Drop lock guards before joining or doing slow work.
- Do not use `unsafe`.

## Review Focus

- Lock ordering discipline.
- Small lock guard scopes.
- Practical deadlock avoidance reasoning.

# 43 Integration Tests

## Goal

Practice testing a public library API from the `tests/` directory.

## Project

Create project:

```bash
cargo new practice/advanced-extra/43_integration_tests --name integration_tests
```

## Requirements

- Create a library crate with `src/lib.rs`.
- Expose a small public API for text normalization and counting.
- Create `tests/text_api.rs`.
- Test only public functions from the crate root.
- Include at least three integration tests.

## Constraints

- Do not access private module internals from integration tests.
- Keep unit tests and integration tests separate.
- Do not use external crates unless needed for test fixtures.
- Run with `cargo test`.

## Review Focus

- Public API boundaries.
- Meaningful integration test cases.
- Avoiding tests coupled to implementation details.

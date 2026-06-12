# 44 Documentation Tests

## Goal

Practice writing examples that are checked by `cargo test`.

## Project

Create project:

```bash
cargo new practice/advanced-extra/44_doc_tests --name doc_tests
```

## Requirements

- Add rustdoc comments to at least two public functions.
- Include runnable code examples in fenced Rust blocks.
- Include one example that demonstrates error handling.
- Run `cargo test` and confirm doc tests pass.

## Constraints

- Examples must compile as users would write them.
- Do not hide required imports unless intentionally using `#` lines.
- Keep examples small and useful.
- Public docs should describe behavior, not implementation details.

## Review Focus

- Accuracy of examples.
- Whether docs teach the public API clearly.
- Doc tests as API regression tests.

# 46 Feature Flags

## Goal

Practice optional functionality controlled by Cargo features.

## Project

Create project:

```bash
cargo new practice/advanced-extra/46_feature_flags --name feature_flags
```

## Requirements

- Start from a small library crate.
- Add optional JSON output support behind a `json` feature.
- When `json` is enabled, use `serde` / `serde_json`.
- When `json` is disabled, crate should still compile without serde.
- Document how to run tests with and without the feature.

## Constraints

- Use `optional = true` dependencies where appropriate.
- Use `#[cfg(feature = "json")]` for feature-gated code.
- Do not require JSON dependencies for the default build.
- Keep the default feature set simple.

## Review Focus

- Correct feature configuration.
- Code that compiles in both feature modes.
- Public API clarity around optional functionality.

# Rust WASM After Advanced

This guide records a mid-term direction after finishing the advanced Rust practice section.

## Positioning

After finishing Advanced, Rust-side prerequisites for WebAssembly should be mostly covered:

- ownership and borrowing
- `Result` and practical error handling
- modules and library structure
- collections and iterators
- closures
- generics and trait bounds
- API signature design
- basic testing

The new learning curve is not mainly Rust syntax. It is the boundary between Rust, WebAssembly, JavaScript, and browser APIs.

## What To Learn Next

Learn these after Advanced:

1. `wasm-bindgen`
2. `wasm-pack`
3. `js-sys`
4. `web-sys`
5. Rust-to-JS type conversion
6. WASM package size and initialization
7. Debugging Rust-generated WASM
8. Calling WASM from a small TypeScript or JavaScript frontend

Only after that, consider Rust frontend frameworks:

- Leptos
- Yew
- Dioxus

The first WASM project should not be a full Rust frontend app. It should be a Rust core library compiled to WASM and called from a normal web page.

## Good First Project

Build:

```text
Rust text analysis core
+ wasm-bindgen wrapper
+ Vite / React / TypeScript demo page
```

Example features:

- word count
- unique word count
- top repeated words
- Markdown heading extraction
- search or filter rules

This fits the existing advanced practice path because it reuses:

- `Vec`
- `HashMap`
- iterators
- closures
- error handling
- module design
- tests

## Where Rust WASM Fits Well

Rust WASM is useful when the browser needs a focused high-performance or reusable core:

- text processing
- parsers
- Markdown or code analysis
- image processing
- compression
- cryptography
- simulation
- rules engines
- search/indexing
- sharing Rust logic across native, server, and browser targets

It is also relevant outside the browser through WASI and component-model work, especially for sandboxed execution and portable modules.

## Where It Is Not Usually Worth It

Rust WASM is usually not the best default for:

- ordinary CRUD pages
- form-heavy admin systems
- simple DOM interactions
- UI work where TypeScript already solves the problem clearly
- using Rust only to replace JavaScript without a performance or reuse reason

For normal web UI, TypeScript remains the practical default. Rust WASM is strongest as a focused module.

## Suggested Path

After finishing Advanced:

1. Build the Stage 1 and Stage 2 text stats projects.
2. Extract the text processing core into a clean library.
3. Compile that library to WASM with `wasm-bindgen`.
4. Call it from a small TypeScript page.
5. Add tests on the Rust side and a minimal browser demo.
6. Only then evaluate Leptos, Yew, or Dioxus.

## References

- Rust and WebAssembly overview: https://www.rust-lang.org/what/wasm
- MDN Rust to WebAssembly guide: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm
- `wasm-bindgen` guide: https://rustwasm.github.io/docs/wasm-bindgen/
- `wasm-pack` documentation: https://rustwasm.github.io/docs/wasm-pack/
- WebAssembly use cases: https://webassembly.org/docs/use-cases/
- Bytecode Alliance Component Model: https://component-model.bytecodealliance.org/

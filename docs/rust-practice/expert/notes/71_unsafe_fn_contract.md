# 71 Unsafe Function Contract - 笔记

## 本题会用到的前置知识点

- unsafe 块与裸指针（68-70 题）。
- 文档注释语法（`///`）。

## 核心结论

- `unsafe fn` 向调用方声明：**你在调用之前必须自己保证这些前提条件，编译器不检查。**
- `# Safety` 文档注释是这份合约的正式书写位置。
- `unsafe fn` ≠ "函数内部有 unsafe 块"；它是一种 API 契约，告知调用者有义务。

## 两种注释形式对比

| 场景 | 写法 |
|---|---|
| `unsafe fn` 的 API 文档 | `/// # Safety` 文档注释（在函数外） |
| 普通函数内部的 unsafe 块 | `// Safety:` 行内注释（在 unsafe 块前） |

## 核心示例

```rust
/// # Safety
///
/// - `ptr` must be non-null.
/// - `ptr` must point to a valid, initialized `i32`.
/// - The pointed-to value must not be aliased by a `&mut i32` at call time.
unsafe fn read_i32(ptr: *const i32) -> i32 {
    *ptr
}

fn main() {
    let x: i32 = 99;
    // Correct: pointer derived from a live local variable.
    let result = unsafe { read_i32(&x as *const i32) };
    println!("{result}");

    // Incorrect (do not run): null pointer violates precondition 1.
    // let _ = unsafe { read_i32(std::ptr::null()) };
}
```

理解重点：调用 `unsafe fn` 时仍然需要 `unsafe {}` 块——调用点是显式承担责任的地方。

## 常见坑

- 写了 `unsafe fn` 但省掉 `# Safety` 文档——调用者无法知道需要满足什么条件。
- 以为 `unsafe fn` 调用时不需要 `unsafe {}` 块——仍然需要，否则编译报错。
- `# Safety` 写得太泛：说"确保指针有效"而不说"什么叫有效"（非空、对齐、生命周期内等）。

## 回看问题

- 如果一个函数内部有 `unsafe {}` 块但函数签名不是 `unsafe fn`，调用方需要写 `unsafe {}` 吗？
- `# Safety` 里的前提条件，违反了会怎样？编译器会报错吗？

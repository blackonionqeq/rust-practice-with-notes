# 80 Unpin Escape - 笔记

## 本题会用到的前置知识点

- `Pin<Box<T>>` 基础（79 题）。
- trait bound 语法。

## 核心结论

- **`Unpin` 是 Rust 里的"默认值"**：几乎所有标准类型（`i32`、`String`、`Vec<T>` 等）都自动实现 `Unpin`。
- 对于 `Unpin` 类型，`Pin<Box<T>>` 不添加任何限制——你仍然可以移动它、获取 `&mut T`。
- 只有显式添加 `PhantomPinned`（或包含 `!Unpin` 字段）的类型才会受到 `Pin` 的真正约束。
- `Pin` 的作用域：保护 `!Unpin` 的类型在 pin 之后不被移动。

## 核心示例

```rust
use std::pin::Pin;

fn main() {
    let mut pinned = Box::pin(42i32);  // i32: Unpin

    // get_mut 对 Unpin 类型可用，无需 unsafe
    let inner: &mut i32 = pinned.as_mut().get_mut();
    *inner = 100;
    println!("{}", pinned);  // 100

    // i32 满足 Unpin bound
    fn requires_unpin<T: Unpin>(_: T) {}
    requires_unpin(0i32);  // 编译通过

    // requires_unpin(Unmovable { ... })  ← 编译失败，Unmovable: !Unpin
}
```

## Unpin 类型 vs !Unpin 类型对比

| 类型 | `Unpin`? | `Pin` 是否有约束 |
|---|---|---|
| `i32`, `String`, `Vec<T>` | ✓ | 无（可自由 get_mut） |
| `async fn` 生成的 Future | ✗ | 有（不可移出） |
| 含 `PhantomPinned` 的 struct | ✗ | 有 |

## 常见坑

- 以为所有放进 `Pin` 的类型都不能移动——只有 `!Unpin` 类型才如此。
- 对 `Unpin` 类型调用 `get_mut` 失败——其实对 `Unpin` 类型可以直接用 `get_mut`，不需要 `unsafe`。

## 回看问题

- 为什么 `async fn` 生成的 Future 通常是 `!Unpin`？（提示：Future 内部可能包含跨 `await` 点的局部变量引用）
- `Pin::get_unchecked_mut` 和 `Pin::get_mut` 有什么区别？

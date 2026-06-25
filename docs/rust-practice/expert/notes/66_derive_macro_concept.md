# 66 Derive Macro Concept - 笔记

## 本题会用到的前置知识点

- struct 定义。
- trait 的基本用途。
- `Debug` 输出与相等比较。

## 核心结论

- `#[derive(...)]` 是编译期代码生成入口。
- 日常最常见价值：快速获得 `Debug/Clone/PartialEq/Eq` 行为。
- 本阶段重点是“会用、会读”，不是手写 proc-macro。

## 核心示例

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let a = User { id: 1, name: "A".into() };
    let b = a.clone();
    println!("{:?}", a);
    println!("{}", a == b);
}
```

理解重点：`derive` 让你不用手写样板 trait 实现。

## 常见坑

- 以为 `derive` 是运行时行为。
- 把“会使用 derive”和“会实现 proc-macro”混为一谈。

## 回看问题

- 这题里 derive 帮你省掉了哪些样板代码？
- 当前阶段真的需要自己写过程宏吗？
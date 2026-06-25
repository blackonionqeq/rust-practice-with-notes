# 68 Unsafe Capabilities - 笔记

## 本题会用到的前置知识点

- 引用与借用基础。
- 函数定义与调用。
- 基本类型（`i32`、`u32` 等）。

## 核心结论

`unsafe` 关键字只解锁五件事，不多不少：

| 能力 | 最简示例 |
|---|---|
| 裸指针解引用 | `unsafe { *p }` |
| 调用 unsafe 函数 | `unsafe { some_unsafe_fn() }` |
| 访问/修改 `static mut` | `unsafe { COUNTER += 1 }` |
| 实现 unsafe trait | `unsafe impl Trusted for T {}` |
| 访问 union 字段 | `unsafe { v.field }` |

在 `unsafe` 块里，其他所有 Rust 规则仍然生效（所有权检查、借用检查等）。`unsafe` 不是"关掉 Rust"，只是"允许这五件事"。

## 核心示例

```rust
let x: i32 = 99;
let p: *const i32 = &x; // 构造裸指针是安全的，不需要 unsafe
unsafe {
    println!("{}", *p);  // 解引用才是 unsafe 能力
}
```

理解重点：构造裸指针本身不需要 `unsafe`；解引用才需要。

## 常见坑

- 误以为 `unsafe` 块里可以违反借用规则——不能。借用检查仍然在工作，只有裸指针操作绕开了它。
- 把 `unsafe fn` 和"函数内部有 unsafe 块"混为一谈：前者是向调用方声明"你需要满足前提条件"，后者只是实现细节。
- 以为 `unsafe impl` 关键字只是语法噪音——它表示"实现者保证满足该 trait 的安全约束"。

## 回看问题

- 构造一个 `*const i32` 需要 unsafe 吗？
- `unsafe impl Trait for T` 是在说谁需要保证什么？
- 同一个 `unsafe` 块里可以同时使用多个 unsafe 能力吗？

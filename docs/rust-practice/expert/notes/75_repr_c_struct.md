# 75 repr(C) Struct Layout - 笔记

## 本题会用到的前置知识点

- struct 定义。
- `std::mem::size_of`。

## 核心结论

- 默认情况下 Rust 编译器可以自由重排 struct 字段以优化布局，不保证顺序。
- `#[repr(C)]` 强制按 C ABI 规则布局：字段顺序与声明顺序一致，对齐遵循 C 规则。
- FFI 边界的类型**必须**使用 `#[repr(C)]`，否则 Rust 侧和 C 侧读到的字段可能不一样。
- `extern "C" { fn foo(...) }` 只是告诉编译器"有这么一个函数存在，签名如此"——链接时才会真正找它。

## 核心示例

```rust
use std::mem::{size_of, offset_of};

#[repr(C)]
struct PointC { x: f32, y: f32, tag: u8 }
struct PointRust { x: f32, y: f32, tag: u8 }

fn main() {
    println!("PointC  size: {}", size_of::<PointC>());   // 通常 12（对齐到 4）
    println!("PointRust size: {}", size_of::<PointRust>()); // 可能也是 12，或 9
    println!("tag offset in PointC: {}", offset_of!(PointC, tag)); // 8
}

extern "C" {
    fn add_floats(a: f32, b: f32) -> f32;
    // 只声明类型，不调用 —— 没有链接到 C 实现时调用会导致链接错误
}
```

## 常见坑

- 以为 Rust 默认 struct 的内存布局"和 C 差不多"——不保证，不能依赖。
- `extern "C"` 块声明了函数但没有链接到实现，调用时会产生链接错误，不是运行时 panic。
- `#[repr(C)]` 和 `#[repr(packed)]` 不一样：packed 会去掉 padding，可能导致未对齐访问。

## 回看问题

- `PointC` 的 `tag` 字段（`u8`）在 9 字节处，为什么 `size_of` 是 12 而不是 9？
- 如果不加 `#[repr(C)]`，Rust 是否保证字段按声明顺序排列？

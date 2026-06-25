# 75 repr(C) Struct Layout - 笔记

## 本题会用到的前置知识点

- struct 定义。
- `std::mem::size_of`。

## 核心结论

- 默认情况下 Rust 编译器可以自由重排 struct 字段以优化布局，不保证顺序。
- `#[repr(C)]` 强制按 C ABI 规则布局：字段顺序与声明顺序一致，对齐遵循 C 规则。
- FFI 边界的类型**必须**使用 `#[repr(C)]`，否则 Rust 侧和 C 侧读到的字段可能不一样。
- `unsafe extern "C" { fn foo(...) }` 只是告诉编译器"有这么一个函数存在，签名如此"——链接时才会真正找它。

## 需要补充看懂的知识

### `size_of` 不等于字段大小简单相加

struct 的大小要同时满足两个条件：

1. 每个字段都放在满足自身对齐要求的位置上。
2. 整个 struct 的大小要是最大字段对齐值的整数倍，方便数组连续存放。

以 `#[repr(C)] struct PointC { x: f32, y: f32, tag: u8 }` 为例：

- `f32` 通常对齐到 4 字节，所以 `x` 在 offset 0，`y` 在 offset 4。
- `tag: u8` 在 offset 8，占 1 字节。
- 字段实际用到 byte 0..=8，一共 9 字节，但整个 struct 仍要按 4 对齐，所以尾部补 3 字节 padding，最终 `size_of::<PointC>()` 通常是 12。

### `offset_of!` 看的是字段起始位置

`offset_of!(PointC, tag)` 返回的是 `tag` 字段从 struct 开头算起的起始偏移，通常是 8，不是 struct 的总大小，也不是字段结束位置。

这点很适合用来检查 FFI 布局：如果 C 侧认为 `tag` 在 offset 8，而 Rust 侧没有 `repr(C)` 导致布局不同，两边就会用不同方式解释同一段内存。

### `repr(C)` 保证布局，不保证所有类型都适合 FFI

`#[repr(C)]` 只解决字段顺序、padding、对齐这些布局问题。它不自动让任意 Rust 类型都变成 C 能理解的类型。

例如 `String`、`Vec<T>`、`Option<String>` 这类类型即使放进 `repr(C)` struct，C 侧也不知道它们的内部约定。FFI 边界应优先使用 C ABI 能表达的字段，例如整数、浮点、裸指针、长度、或其他 `repr(C)` 类型。

### Rust 2024 里 `extern` block 要写 `unsafe`

`unsafe extern "C" { ... }` 的 `unsafe` 表示：这些外部函数的声明正确性无法由 Rust 编译器验证，写声明的人要负责保证签名、ABI、链接名称和真实 C 实现一致。

这不等于"声明时调用了 unsafe 代码"。真正调用外部函数时仍然需要在 `unsafe` block 里调用，因为 Rust 无法检查 C 函数是否满足 Rust 的安全约定。

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

unsafe extern "C" {
    fn add_floats(a: f32, b: f32) -> f32;
    // 只声明类型，不调用 —— 没有链接到 C 实现时调用会导致链接错误
}
```

## 常见坑

- 以为 Rust 默认 struct 的内存布局"和 C 差不多"——不保证，不能依赖。
- `extern "C"` 块声明了函数但没有链接到实现，调用时会产生链接错误，不是运行时 panic。
- `#[repr(C)]` 和 `#[repr(packed)]` 不一样：packed 会去掉 padding，可能导致未对齐访问。

## 回看问题

- `PointC` 的 `tag` 字段（`u8`）起始 offset 是 8，字段内容结束后实际用到 9 字节，为什么 `size_of` 是 12 而不是 9？
- 如果不加 `#[repr(C)]`，Rust 是否保证字段按声明顺序排列？

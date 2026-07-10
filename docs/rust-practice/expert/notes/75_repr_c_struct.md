# 75 repr(C) Struct Layout - 笔记

## 本题会用到的前置知识点

- struct 定义。
- `std::mem::size_of`。

## 为什么现在学习 FFI

第 68–74 题学习了 unsafe、裸指针、安全约定以及如何缩小 unsafe 范围。本题开始观察这些知识最常见的实际用途之一：Rust 与其他语言或外部二进制代码交互，也就是 FFI。

跨语言交互时，边界两侧可能需要调用对方的函数，或者按照相同方式解释一段内存。为此，双方必须约定：

- 函数如何传递参数和返回值。
- 数据的字段顺序、大小、对齐和 padding。

本题只学习这类边界的类型和布局，不会真正编译或链接 C 代码。`extern "C"` 用来展示函数调用边界，`#[repr(C)]` 用来展示数据布局边界；完整的 FFI 工程实践不在本题范围内。

## 核心结论

- 默认情况下 Rust 编译器可以自由重排 struct 字段以优化布局，不保证顺序。
- `#[repr(C)]` 按当前编译目标的 C ABI 布局数据：字段顺序与声明顺序一致，并按字段对齐要求插入 padding。
- 当 FFI 两侧要按照 C struct 的字段 offset 直接解释同一段内存时，Rust 类型应使用 `#[repr(C)]`。
- `unsafe extern "C" { fn foo(...) }` 只是告诉编译器"有这么一个函数存在，签名如此"——链接时才会真正找它。

## 需要补充看懂的知识

### FFI、ABI 和 `repr(C)` 分别是什么

FFI 是 Foreign Function Interface（外部函数接口），指一种语言调用另一种语言编译出的函数、交换数据的边界。这里的“外部”不只指 C；只是很多语言都能提供 C 接口，所以 C ABI 经常被用作不同语言之间的公共边界。

ABI 是 Application Binary Interface（二进制接口），约定已经编译好的代码如何协作，例如：

- 函数参数和返回值如何传递。
- 函数符号如何命名和链接。
- 数据在内存中的大小、对齐和布局。

`extern "C"` 选择的是函数调用层面的 C ABI，`#[repr(C)]` 选择的是类型布局层面的 C representation。两者解决的问题不同：前者管“函数怎么调用”，后者管“数据在内存里是什么样”。

`#[repr(C)]` 也不只是“改成 C 的对齐方式”。以 struct 为例，它共同确定：

- 字段保持声明顺序。
- 每个字段的起始 offset 满足该字段的 alignment。
- 字段之间需要插入多少 padding。
- struct 自身的 alignment 和尾部 padding。

这些结果仍与编译目标有关。同一个 `#[repr(C)]` 类型在 `x86_64` 和 `wasm32` 上不一定具有相同大小，因此不能把它理解成一套跨所有平台都固定不变的字节格式。

### `size_of` 不等于字段大小简单相加

struct 的大小要同时满足两个条件：

1. 每个字段都放在满足自身对齐要求的位置上。
2. 整个 struct 的大小要是最大字段对齐值的整数倍，方便数组连续存放。

以 `#[repr(C)] struct DemoC { a: u8, b: u64, c: u8 }` 为例，在 `u64` 按 8 字节对齐的常见 64 位目标上：

- `a: u8` 在 offset 0，占 1 字节。
- 为了让 `b: u64` 从 8 的整数倍地址开始，`a` 后面插入 7 字节 padding，`b` 在 offset 8。
- `c: u8` 在 offset 16，占 1 字节。
- struct 的 alignment 是 8，因此末尾再补 7 字节 padding，`size_of::<DemoC>()` 是 24。

默认布局的 `DemoRust` 不要求保持声明顺序。当前编译器可以把 `u64` 放在前面，再把两个 `u8` 放在一起，从而让它在同一目标上只占 16 字节。这正是本题选择 `u8, u64, u8` 的原因：字段顺序留下了明显的优化空间。

不过，16 字节以及每个字段的具体 offset 都只是本次编译的观察结果，不是默认 Rust 布局的稳定承诺。可以依赖的是 `DemoC` 的 C representation 规则，不能把当前 `DemoRust` 的排列写进跨语言协议。

### `offset_of!` 看的是字段起始位置

`offset_of!(DemoC, c)` 返回的是 `c` 字段从 struct 开头算起的起始偏移，在上述目标上是 16，不是 struct 的总大小，也不是字段结束位置。

这点很适合用来检查 FFI 布局：`DemoC` 的 `b` 和 `c` 分别位于 offset 8 和 16；`DemoRust` 的观察值可能不同。如果外部代码按照 `DemoC` 的 offset 读取 `DemoRust`，双方就会用不同方式解释同一段内存。

### `repr(C)` 保证布局，不保证所有类型都适合 FFI

`#[repr(C)]` 只解决字段顺序、padding、对齐这些布局问题。它不自动让任意 Rust 类型都变成 C 能理解的类型。

例如 `String`、`Vec<T>`、`Option<String>` 这类类型即使放进 `repr(C)` struct，C 侧也不知道它们的内部约定。FFI 边界应优先使用 C ABI 能表达的字段，例如整数、浮点、裸指针、长度、或其他 `repr(C)` 类型。

“FFI 类型必须使用 `repr(C)`”只适用于双方要按 C struct 布局直接解释同一段内存的情况。只传递普通数值、通过不透明指针操作对象，或者先把数据序列化，都不要求暴露 Rust struct 的字段布局。

### 其他语言以及 Wasm/JavaScript 怎么处理布局

跨语言交互的关键不是所有语言采用同一种对齐方式，而是边界两侧遵守同一份协议。常见方案有：

1. 双方采用 C ABI：Rust 使用 `extern "C"` 和必要的 `#[repr(C)]`，另一种语言通过自己的 C FFI 与之对应。
2. 使用绑定生成器或桥接层：桥接代码负责把两边的类型互相转换。
3. 使用 JSON、Protocol Buffers 等格式序列化数据，不共享进程内 struct 布局。

Rust 编译成 WebAssembly 后交给 JavaScript 调用时，通常使用 `wasm-bindgen`。JavaScript 没有与 Rust struct 对应的原生内存布局；生成的绑定一般把 Rust 对象留在 Wasm 线性内存中，JavaScript 只保存一个不透明的指针，并通过生成的 getter、setter 或方法访问它。因此这种用法通常不需要给导出的 Rust struct 添加 `#[repr(C)]`。

字符串、数组和复杂对象也通常由绑定层转换，或者使用 `serde-wasm-bindgen` 转成 JavaScript 原生值。只有在手动返回 pointer/length，让 JavaScript 通过 `DataView` 或 TypedArray 直接读取 Wasm 线性内存时，才需要显式约定字段 offset、大小和编码；这时可以使用 `#[repr(C)]` 作为布局协议的一部分，但 JavaScript 必须按照这份协议读取，并且不能假设另一个编译目标也具有相同布局。

### Rust 2024 里 `extern` block 要写 `unsafe`

`unsafe extern "C" { ... }` 的 `unsafe` 表示：这些外部函数的声明正确性无法由 Rust 编译器验证，写声明的人要负责保证签名、ABI、链接名称和真实 C 实现一致。

这不等于"声明时调用了 unsafe 代码"。真正调用外部函数时仍然需要在 `unsafe` block 里调用，因为 Rust 无法检查 C 函数是否满足 Rust 的安全约定。

## 核心示例

```rust
use std::mem::{size_of, offset_of};

#[repr(C)]
struct DemoC { a: u8, b: u64, c: u8 }
struct DemoRust { a: u8, b: u64, c: u8 }

fn main() {
    println!("DemoC size: {}", size_of::<DemoC>());       // 常见 64 位目标上是 24
    println!("DemoRust size: {}", size_of::<DemoRust>()); // 当前编译器可能得到 16
    println!("DemoC.b offset: {}", offset_of!(DemoC, b));
    println!("DemoC.c offset: {}", offset_of!(DemoC, c));
    println!("DemoRust.b offset: {}", offset_of!(DemoRust, b));
    println!("DemoRust.c offset: {}", offset_of!(DemoRust, c));
}

unsafe extern "C" {
    fn add_bytes(a: u8, b: u8) -> u8;
    // 只声明类型，不调用 —— 没有链接到 C 实现时调用会导致链接错误
}
```

## 常见坑

- 以为 Rust 默认 struct 的内存布局"和 C 差不多"——不保证，不能依赖。
- 以为 `repr(C)` 会生成跨所有 CPU 和操作系统都相同的字节格式——它仍取决于当前编译目标的类型大小和对齐。
- 以为只要跨语言就必须给所有 struct 加 `repr(C)`——使用绑定转换、序列化或不透明指针时，并不共享字段布局。
- `extern "C"` 块声明了函数但没有链接到实现，调用时会产生链接错误，不是运行时 panic。
- `#[repr(C)]` 和 `#[repr(packed)]` 不一样：packed 会去掉 padding，可能导致未对齐访问。

## 回看问题

- 在常见 64 位目标上，`DemoC` 的字段内容只用到前 17 字节，为什么 `size_of` 是 24 而不是 17？
- 为什么当前编译器可能把字段相同的 `DemoRust` 布局成 16 字节？这个结果可以作为稳定契约吗？
- 如果不加 `#[repr(C)]`，Rust 是否保证字段按声明顺序排列？
- `extern "C"` 和 `#[repr(C)]` 分别约束哪一层？
- 使用 `wasm-bindgen` 把 Rust struct 暴露给 JavaScript 时，JavaScript 是否通常会直接读取该 struct 的字段 offset？

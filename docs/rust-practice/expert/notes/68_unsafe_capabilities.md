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

### 什么场景下才需要 unsafe

实际写 Rust 时，优先假设“不需要 unsafe”。只有当某些安全前提编译器无法证明、但你能在更高层清楚证明时，才考虑把很小一段实现写成 unsafe。

常见正当场景包括：

- **FFI 边界**：调用 C 函数、处理 C 字符串指针、保证 `#[repr(C)]` 布局和真实 ABI 一致。
- **实现安全抽象的内部细节**：标准库容器、迭代器、智能指针、同步原语等，经常在内部用 unsafe 维护不变量，对外提供 safe API。
- **底层内存和性能控制**：手动内存布局、已由外层检查证明过的指针偏移、避免重复 bounds check、SIMD 或系统调用边界。
- **全局可变状态或底层并发原语**：通常先用 `OnceLock`、`Mutex`、原子类型；只有封装更底层机制时才考虑 unsafe。
- **地址稳定性和自引用结构**：例如理解 `Pin`、async runtime、intrusive data structure 时会遇到，但日常业务代码应优先找 safe 替代方案。

判断原则：unsafe 不是为了“写起来方便”，而是因为有些事实 Rust 编译器看不见，但你能通过 API 设计、前置检查和不变量说明把它证明清楚。

## 需要补充看懂的知识

### `unsafe` 表示“编译器不再替你证明这一小段”

安全 Rust 的核心价值是：很多内存安全前提由编译器检查。写 `unsafe` 时，不是规则消失了，而是其中一部分前提改由程序员证明。

例如：

```rust
let x = 99;
let p: *const i32 = &x;
unsafe {
	println!("{}", *p);
}
```

编译器不会在 `*p` 这一行继续证明 `p` 一定有效、对齐、指向已初始化的 `i32`。你需要根据 `p` 的来源说明为什么成立。

### UB 不是 panic

unsafe 里违反安全前提通常导致 undefined behavior（UB），不是普通错误返回，也不保证 panic。

UB 的意思是：程序行为不再受 Rust 语义保证。它可能看起来能跑，也可能输出奇怪结果，也可能在优化后出错。不要用“我运行了一次没崩”来证明 unsafe 是正确的。

### unsafe 块不是隔离舱

下面这些仍然受普通 Rust 规则约束：

- 变量所有权移动。
- 普通引用的借用检查。
- 类型检查。
- 可见性和模块规则。

`unsafe` 只是允许你执行五类操作。它不会让错误类型通过编译，也不会让普通 `&mut` aliasing 变得合法。

### unsafe 的两种责任位置

读 unsafe 代码时先分清责任在谁身上：

- `unsafe { ... }`：当前代码块作者要证明块内操作满足前提。
- `unsafe fn f(...)`：调用者要在调用前满足函数文档里的 `# Safety` 合约。

第 68 题只要求识别五种能力。后面 69-74 会逐步练习如何写证明、如何把责任封装进 safe wrapper。

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
- 把 UB 当成 panic 或 `Result` 错误处理。
- 用“测试能跑过”代替安全性证明。
- 为了绕过借用检查或图省事而使用 unsafe——这通常说明 API 设计还没想清楚，应该先找 safe 表达方式。

## 回看问题

- 构造一个 `*const i32` 需要 unsafe 吗？
- `unsafe impl Trait for T` 是在说谁需要保证什么？
- 同一个 `unsafe` 块里可以同时使用多个 unsafe 能力吗？
- 这段 unsafe 的前提条件由谁负责证明？

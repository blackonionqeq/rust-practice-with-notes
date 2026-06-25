# 69 Raw Pointer Read - 笔记

## 本题会用到的前置知识点

- 切片与引用。
- unsafe 的五个能力（68 题）。

## 核心结论

- 裸指针的来源决定它是否安全：从有效引用派生的裸指针，在引用的生命周期内解引用是安全的。
- **构造裸指针是安全操作；解引用才是 unsafe 操作。**
- 编译器相信你：它不检查指针是否有效，由你在注释中写明前提条件。

## 需要补充看懂的知识

### 裸指针和引用的差别

引用 `&T` 带着 Rust 的借用保证：非空、对齐、指向有效初始化数据，并受生命周期约束。

裸指针 `*const T` 没有这些自动保证。它可以是 null，也可以悬垂，也可以指向未初始化内存。创建裸指针不危险，危险发生在你尝试解引用它时。

```rust
let x = 10;
let p: *const i32 = &x;
```

这行是安全的，因为只是把引用转成裸指针。真正需要证明的是：

```rust
unsafe { *p }
```

### `as_ptr()` 对空切片也安全

```rust
let empty: &[u8] = &[];
let p = empty.as_ptr();
```

这本身安全。切片即使为空，也能给出一个可作为地址计算起点的指针。但空切片没有第一个元素，所以解引用 `p` 是 UB。

因此第 69 题的函数：

```rust
fn first_byte(data: &[u8]) -> u8
```

必须要求调用方保证 `data` 非空。因为这个函数返回 `u8`，没有 `Option` 表示失败，所以它暂时把责任留给调用方。第 72 题会把这个责任收回到函数内部。

### 前提条件要写在 unsafe 前面

本题不是只练语法，还练“证明习惯”：

```rust
// Safety: data is non-empty, so data.as_ptr() points to the first initialized byte.
unsafe { *p }
```

好的注释应该说明：

- 需要什么条件。
- 这个条件从哪里来。
- 这个条件为什么足够支撑当前 unsafe 操作。

### 不要从整数地址造指针

```rust
// 错误思路：来源不明，不能证明这个地址有效
// let p = 42usize as *const i32;
```

初学阶段只从有效引用、切片、`Box`、`Vec` 这类 Rust 已经管理的数据结构中派生裸指针。这样你至少能解释指针来源和生命周期。

## 核心示例

```rust
fn first_byte(data: &[u8]) -> u8 {
    // Precondition: data must be non-empty.
    let p: *const u8 = data.as_ptr(); // safe: just casting a reference
    // Safety: data is non-empty (caller's responsibility here).
    unsafe { *p }
}
```

理解重点：`as_ptr()` 是普通方法调用；`*p` 才进入 unsafe 边界。

## 常见坑

- 用 `42usize as *const i32` 之类的强制转换——这样构造的指针来源不明，几乎必然是 undefined behavior。
- 忘记写前提条件注释，让"为什么可以这样做"成为读代码时的谜题。
- 在空切片上调用 `as_ptr()` 是安全的（返回悬空但对齐的指针），但解引用这个指针是 UB。
- 以为 `*const T` 只能读，所以一定安全。读无效地址同样是 UB。
- 把“指针不是 null”误认为足够安全；还需要有效、对齐、已初始化、在生命周期内。

## 回看问题

- 如果 `data` 是空切片，`data.as_ptr()` 返回什么？解引用它安全吗？
- 这道题的前提条件目前是靠什么来保证的？（目前靠调用方自己注意——72 题会用 `Option` 解决这个问题）
- 这个裸指针是从什么有效数据派生出来的？

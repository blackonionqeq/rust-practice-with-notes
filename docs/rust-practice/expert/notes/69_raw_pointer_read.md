# 69 Raw Pointer Read - 笔记

## 本题会用到的前置知识点

- 切片与引用。
- unsafe 的五个能力（68 题）。

## 核心结论

- 裸指针的来源决定它是否安全：从有效引用派生的裸指针，在引用的生命周期内解引用是安全的。
- **构造裸指针是安全操作；解引用才是 unsafe 操作。**
- 编译器相信你：它不检查指针是否有效，由你在注释中写明前提条件。

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

## 回看问题

- 如果 `data` 是空切片，`data.as_ptr()` 返回什么？解引用它安全吗？
- 这道题的前提条件目前是靠什么来保证的？（目前靠调用方自己注意——72 题会用 `Option` 解决这个问题）

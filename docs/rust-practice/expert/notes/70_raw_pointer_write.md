# 70 Raw Pointer Write - 笔记

## 本题会用到的前置知识点

- 可变引用与切片。
- 裸指针读（69 题）。

## 核心结论

- `*mut T` 允许写入，但你必须保证三件事：目标地址有效、正确对齐、没有其他引用同时访问同一位置。
- `// Safety:` 注释要写具体条件，不能写"trust me"或"it's fine"。
- 从 `&mut [T]` 派生的 `*mut T` 天然满足"无其他引用同时访问"——因为可变引用本身就保证了独占性。

## 核心示例

```rust
fn negate_first(data: &mut [i32]) {
    if data.is_empty() { return; }
    let p: *mut i32 = data.as_mut_ptr();
    // Safety: data is non-empty (checked above), so p points to a valid,
    // aligned i32. The &mut reference guarantees exclusive access.
    unsafe {
        *p = -(*p);
    }
}
```

理解重点：`&mut [i32]` 的存在本身就是"无 aliasing"的证明；你只需要再证明"非空"。

## 常见坑

- 同时持有 `&mut data[0]` 和 `*mut i32` 指向同一位置——这违反 aliasing 规则，是 UB。
- `// Safety:` 写成"checked above"而不指向具体的哪个检查。
- 读写混写在多个语句里，不确定哪个语句触发了 unsafe——本题只有一个写操作，保持简单。

## 回看问题

- 为什么从 `&mut [i32]` 派生的 `*mut i32` 不会产生 aliasing 问题？
- 如果在 `unsafe` 块里先读后写（两个操作），Safety 注释应该写什么？

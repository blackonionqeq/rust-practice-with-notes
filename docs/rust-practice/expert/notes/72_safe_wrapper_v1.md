# 72 Safe Wrapper v1 - 笔记

## 本题会用到的前置知识点

- 裸指针读（69 题）。
- `unsafe fn` contract 概念（71 题）。
- `Option` 返回值。

## 核心结论

- **safe wrapper 模式**：在进入 `unsafe` 之前，在安全代码里完成所有前提检查，然后带着"这些条件已成立"的保证进入 `unsafe`。
- `// Safety:` 注释要指向具体的哪个检查，而不是泛泛说"已经检查过了"。
- 调用方看到的是普通 safe 函数，不需要知道内部有 unsafe——这就是封装的价值。

## 核心示例

```rust
pub fn first_byte(data: &[u8]) -> Option<u8> {
    if data.is_empty() {
        return None;
    }
    let p: *const u8 = data.as_ptr();
    // Safety: data is non-empty (checked above), so p is valid and non-null.
    let byte = unsafe { *p };
    Some(byte)
}
```

理解重点：`if data.is_empty()` 这一行就是整个安全保证的来源；`// Safety:` 注释只是把这个事实显式写出来。

## 常见坑

- 把 safe wrapper 写成 `unsafe fn`——这样调用方还是需要承担责任，封装的意义就消失了。
- `// Safety:` 写在 unsafe 块之后，或者写在离 unsafe 块很远的地方——注释要紧贴 unsafe 块。
- 检查和进入 unsafe 之间插入了可能改变状态的代码，使得"条件已成立"的保证失效。

## 回看问题

- 如果去掉 `if data.is_empty()` 检查，这个函数还能保持 safe 签名吗？
- 69 题里的 `first_byte` 和本题的 `first_byte` 有什么本质区别？

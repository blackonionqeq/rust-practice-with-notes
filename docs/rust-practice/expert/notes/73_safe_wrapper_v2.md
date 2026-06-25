# 73 Safe Wrapper v2 - 笔记

## 本题会用到的前置知识点

- safe wrapper 模式（72 题）。
- 切片边界检查。
- 单元测试基础（`#[cfg(test)]`、`#[test]`）。

## 核心结论

- 多个不变量时，每个都要在进入 `unsafe` 之前单独检查，并在 `// Safety:` 注释里逐条列出。
- 测试应覆盖"边界"而不只是正常路径：空切片、恰好越界、正好在边界内。
- `ptr.add(n)` 是 unsafe 操作，因为编译器不知道 `n` 是否在有效范围内——你在进入 unsafe 之前做的检查就是证明。

## 核心示例

```rust
pub fn read_at(data: &[i32], index: usize) -> Option<i32> {
    if data.is_empty() || index >= data.len() {
        return None;
    }
    let p: *const i32 = data.as_ptr();
    // Safety:
    // 1. data is non-empty (checked above), so p is valid and non-null.
    // 2. index < data.len() (checked above), so p.add(index) is in bounds.
    let val = unsafe { *p.add(index) };
    Some(val)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn reads_valid_index()  { assert_eq!(read_at(&[10, 20, 30], 1), Some(20)); }
    #[test] fn out_of_bounds()      { assert_eq!(read_at(&[10], 5), None); }
    #[test] fn empty_slice()        { assert_eq!(read_at(&[], 0), None); }
}
```

理解重点：两个 `// Safety:` 条目对应两个不同的检查——缺少任何一个都会让 `*p.add(index)` 成为 UB。

## 常见坑

- 先做 `ptr.add(index)` 再做边界检查——顺序颠倒，越界写入已经发生了。
- 只测试 `Some` 路径，漏掉 `None` 的两种情况（越界和空切片）。
- 把两个 `if` 检查合并成一行后，忘记更新 `// Safety:` 注释。

## 回看问题

- 这里的测试是在验证"unsafe 指针操作正确"还是"safe 边界条件正确"？（答案是后者）
- 如果 `data.len()` 为 0，那 `p` 本身（未解引用）有问题吗？

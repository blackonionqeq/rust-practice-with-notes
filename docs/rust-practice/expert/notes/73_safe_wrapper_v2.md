# 73 Safe Wrapper v2 - 笔记

## 本题会用到的前置知识点

- safe wrapper 模式（72 题）。
- 切片边界检查。
- 单元测试基础（`#[cfg(test)]`、`#[test]`）。

## 核心结论

- 多个不变量时，每个都要在进入 `unsafe` 之前单独检查，并在 `// Safety:` 注释里逐条列出。
- 测试应覆盖"边界"而不只是正常路径：空切片、恰好越界、正好在边界内。
- `ptr.add(n)` 是 unsafe 操作，因为编译器不知道 `n` 是否在有效范围内——你在进入 unsafe 之前做的检查就是证明。

### 现实中哪里会见到

真实 unsafe 封装通常不是一个前提，而是一组前提共同成立：指针来源、长度、索引范围、初始化状态、别名关系都可能影响安全性。

第 73 题的重点就是练习这种多不变量封装。每多一个不变量，都要有对应的检查、注释和测试边界，不能只写一句“checked above”。

## 需要补充看懂的知识

### `ptr.add(index)` 的安全含义

`ptr.add(index)` 不是普通整数加法。它表示从 `ptr` 指向的元素开始，向后移动 `index` 个 `i32` 元素。

要让下面这行安全：

```rust
let val = unsafe { *p.add(index) };
```

你需要证明两件事：

- `p` 来自同一个切片的起点。
- `index < data.len()`，所以目标位置仍在切片内。

如果 `index == data.len()`，那是 one-past-the-end 位置，可以用于某些指针计算场景，但不能解引用。本题不进入这些高级场景，只记住：要读值，必须严格小于 `len`。

### 空切片检查和越界检查可以合并，但证明要完整

代码可以写成：

```rust
if index >= data.len() {
	return None;
}
```

这个检查已经覆盖空切片：当 `len == 0` 时，任何 `usize` index 都满足 `index >= 0`。

练习里写 `data.is_empty() || index >= data.len()` 是为了让两个不变量更显眼。无论代码怎么写，`// Safety:` 里都要说清楚：

- 目标元素存在。
- `p.add(index)` 不越界。

### 测试验证 safe 边界，不是证明 unsafe 正确

单元测试应该覆盖 wrapper 的输入行为：

```rust
assert_eq!(read_at(&[10, 20, 30], 1), Some(20));
assert_eq!(read_at(&[10], 1), None);
assert_eq!(read_at(&[], 0), None);
```

这些测试不能“证明没有 UB”，但能防止你破坏 safe 边界检查。例如误删 `index >= data.len()` 时，边界测试会暴露问题。

### 安全证明要跟代码同步

如果你把检查改写成 helper：

```rust
if !in_bounds(data, index) {
	return None;
}
```

那么 `// Safety:` 不能还写“checked above”就结束。它应说明 `in_bounds` 具体保证了什么，否则审查 unsafe 时还要跳转猜测。

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

- 先做 `ptr.add(index)` 再做边界检查——顺序颠倒，越界指针计算或读取已经发生了。
- 只测试 `Some` 路径，漏掉 `None` 的两种情况（越界和空切片）。
- 把两个 `if` 检查合并成一行后，忘记更新 `// Safety:` 注释。
- 把 `index == data.len()` 当成可读位置。
- 测试只看输出，不检查边界路径是否仍然不进入 unsafe。

## 回看问题

- 这里的测试是在验证"unsafe 指针操作正确"还是"safe 边界条件正确"？（答案是后者）
- 如果 `data.len()` 为 0，那 `p` 本身（未解引用）有问题吗？
- 哪个检查证明 `p.add(index)` 可以被解引用？

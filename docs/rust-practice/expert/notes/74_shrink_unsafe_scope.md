# 74 Shrink Unsafe Scope - 笔记

## 本题会用到的前置知识点

- safe wrapper 模式（72-73 题）。
- 代码阅读与小规模重构。

## 核心结论

- **unsafe 块应该尽可能小**：只包含必须是 unsafe 的那几行，其余全部移出去。
- 移出去的东西：边界检查、变量声明、`as_ptr()` 调用、返回值构造——这些都是安全操作。
- 小 unsafe 块的好处：审查成本低，`// Safety:` 注释可以精准描述，不会遗漏条件。

## 核心示例

```rust
// Before: unsafe block contains safe operations (control flow, ptr construction)
fn sum_pair_before(data: &[i32], i: usize, j: usize) -> Option<i32> {
    unsafe {
        if i >= data.len() || j >= data.len() { return None; }
        let p = data.as_ptr();
        let a = *p.add(i);
        let b = *p.add(j);
        Some(a + b)
    }
}

// After: unsafe block contains only the two dereferences
fn sum_pair(data: &[i32], i: usize, j: usize) -> Option<i32> {
    if i >= data.len() || j >= data.len() {
        return None;
    }
    let p = data.as_ptr();
    // Safety: i < data.len() and j < data.len() (checked above).
    let (a, b) = unsafe { (*p.add(i), *p.add(j)) };
    Some(a + b)
}
```

理解重点：重构前后功能完全相同；区别只在于"需要人工审查的范围缩小了"。

## 常见坑

- 觉得"反正整个函数都在 unsafe 块里，多放点无所谓"——这让审计工作量指数级增长。
- 把 `data.as_ptr()` 也放进 unsafe——没必要，它是安全操作。
- 重构后忘记更新 `// Safety:` 注释，导致注释与实际不符。

## 回看问题

- 这道题的两个 `*p.add(n)` 能完全替换成 `data[i]` 和 `data[j]` 吗？替换后还需要 unsafe 吗？
- 如果一个 unsafe 块里有 15 行代码，怎么判断哪些可以移出去？

# 74 Shrink Unsafe Scope - 笔记

## 本题会用到的前置知识点

- safe wrapper 模式（72-73 题）。
- 代码阅读与小规模重构。

## 核心结论

- **unsafe 块应该尽可能小**：只包含必须是 unsafe 的那几行，其余全部移出去。
- 移出去的东西：边界检查、变量声明、`as_ptr()` 调用、返回值构造——这些都是安全操作。
- 小 unsafe 块的好处：审查成本低，`// Safety:` 注释可以精准描述，不会遗漏条件。

## 需要补充看懂的知识

### 缩小 unsafe 不是机械移动代码

目标不是为了让 unsafe 块“看起来短”，而是让需要人工证明的代码范围更准确。

安全操作可以移出去：

- `if i >= data.len()` 这类边界检查。
- `let p = data.as_ptr()`。
- `Some(a + b)`。

真正需要 unsafe 的是：

```rust
*p.add(i)
*p.add(j)
```

因为这两行涉及裸指针偏移和解引用。

### 证明链不能被移动断开

重构后要能按顺序读出证明：

1. 检查 `i < data.len()` 和 `j < data.len()`。
2. 从同一个 `data` 取出 `p`。
3. 进入 unsafe，只读取 `p.add(i)` 和 `p.add(j)`。

如果在检查和 unsafe 之间插入会改变 `data`、`i`、`j` 含义的代码，证明链就变弱了。本题保持它们不变。

### 一个 unsafe 块可以包含多个同类操作

最终版本：

```rust
let (a, b) = unsafe { (*p.add(i), *p.add(j)) };
```

这个 unsafe 块里有两个读取。它仍然可以接受，因为两者共享同一组已经检查过的前提：两个 index 都在 bounds 内。

如果两个 unsafe 操作依赖完全不同的条件，通常更适合拆成两个小块，并分别写 `// Safety:`。

### 能不用 unsafe 时当然更好

本题故意保留裸指针，是为了练习缩小 unsafe 范围。实际业务代码里，这个函数可以直接写成：

```rust
fn sum_pair(data: &[i32], i: usize, j: usize) -> Option<i32> {
	Some(*data.get(i)? + *data.get(j)?)
}
```

这版完全不需要 unsafe。第 74 题的学习目标不是“这里必须用 unsafe”，而是“当你遇到 unsafe 时，如何把它压到最小并说明前提”。

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
- 只缩短块长度，却让前提检查离 unsafe 太远。
- 把依赖不同前提的 unsafe 操作硬塞进同一个块。

## 回看问题

- 这道题的两个 `*p.add(n)` 能完全替换成 `data[i]` 和 `data[j]` 吗？替换后还需要 unsafe 吗？
- 如果一个 unsafe 块里有 15 行代码，怎么判断哪些可以移出去？
- 现在的 `// Safety:` 是否能逐条对应 unsafe 块里的每个指针读取？

# 17 Closure Filter - 补充笔记

## 本题覆盖

- 闭包捕获环境变量。
- 闭包作为函数参数。
- `Fn` trait bound。
- 返回引用集合。

## 需要补充

### 闭包可以捕获外部变量

```rust
let min_priority = 3;
let result = filter_tasks(&tasks, |task| task.priority >= min_priority);
```

闭包里使用了外部的 `min_priority`，这叫捕获环境。

### `Fn` / `FnMut` / `FnOnce`

闭包根据如何使用捕获变量，自动实现一个或多个 trait：

- `Fn`：只读捕获变量，可调用多次。
- `FnMut`：会修改捕获变量，需要可变调用。
- `FnOnce`：会消费捕获变量，只能调用一次。

本题的 predicate 只读任务和阈值，所以用：

```rust
F: Fn(&Task) -> bool
```

### 返回引用避免克隆任务

```rust
fn filter_tasks<F>(tasks: &[Task], predicate: F) -> Vec<&Task>
```

返回的 `&Task` 都借用自输入 `tasks`。只要 `tasks` 还活着，这些引用就有效。

这个签名避免了复制或移动 `Task`。

### 生命周期可以被推断

完整关系大致是：返回的任务引用生命周期来自 `tasks` 参数。这个简单模式通常可以由编译器推断，不需要手写生命周期。

如果编译器无法推断，可能需要写成：

```rust
fn filter_tasks<'a, F>(tasks: &'a [Task], predicate: F) -> Vec<&'a Task>
where
    F: Fn(&Task) -> bool,
```

本题先使用省略版。

## 常见坑

- 返回 `Vec<Task>`，导致想 clone 或移动任务。
- trait bound 写成 `Fn(Task) -> bool`，这会尝试消费任务。
- 闭包捕获变量后，又在不合适的位置移动该变量。
- 不理解返回引用依赖输入 `tasks` 的生命周期。

## 回看问题

- 闭包捕获和普通函数参数有什么区别？
- `Fn`、`FnMut`、`FnOnce` 的核心区别是什么？
- 为什么本题返回 `Vec<&Task>` 更合适？

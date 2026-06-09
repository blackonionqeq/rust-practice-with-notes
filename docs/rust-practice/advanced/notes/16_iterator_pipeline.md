# 16 Iterator Pipeline - 补充笔记

## 本题覆盖

- 迭代器适配器。
- 惰性执行。
- `map` / `filter` / `collect`。
- `sum`。
- 字节长度和字符数量。

## 需要补充

### 迭代器链通常是惰性的

`map`、`filter` 这类方法不会立刻执行完整计算，它们只是构造新的迭代器。

真正触发执行的是消费方法，例如：

- `collect`
- `sum`
- `count`
- `for_each`

### `iter`、`into_iter`、`iter_mut`

常见区别：

- `.iter()`：产生不可变引用。
- `.iter_mut()`：产生可变引用。
- `.into_iter()`：消费集合，产生拥有所有权的值。

本题输入是 `&[&str]`，不应消费原数据，所以用 `.iter()`。

### `map` 负责转换，`filter` 负责保留

典型流程：

```rust
words
    .iter()
    .map(|word| word.trim().to_lowercase())
    .filter(|word| word.chars().count() >= 3)
    .collect()
```

注意 `to_lowercase()` 会返回新的 `String`。

### `collect` 需要目标类型

当返回类型已经写明是 `Vec<String>` 时，编译器通常能推断：

```rust
collect()
```

如果推断不出来，可以写：

```rust
collect::<Vec<String>>()
```

### 字符数量仍然用 `.chars().count()`

进阶阶段仍要记住：`.len()` 是字节数。过滤单词长度时，如果题目说“字符数”，就用 `.chars().count()`。

## 常见坑

- 写了 `map` / `filter`，但没有 `collect` 或其他消费方法。
- 在需要保留原集合时使用 `.into_iter()`。
- `filter` 闭包参数多了一层引用时不清楚类型。
- 用 `.len()` 判断中文或 Unicode 文本长度。

## 回看问题

- 哪些迭代器方法是惰性的，哪些会消费？
- `.iter()` 和 `.into_iter()` 对所有权有什么影响？
- 为什么 `collect` 有时需要类型标注？

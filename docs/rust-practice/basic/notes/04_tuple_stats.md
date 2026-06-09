# 04 Tuple Stats - 补充笔记

## 本题覆盖

- 接收 `String` 所有权。
- 返回元组，把所有权交还给调用方。
- 同时计算字节长度和字符数量。
- 元组表达式的求值顺序。

## 需要补充

### 返回所有权是一种设计选择

如果函数签名是：

```rust
fn analyze(text: String) -> (String, usize, usize)
```

它表达的是：

- 调用方把 `String` 交给函数。
- 函数可以读取、处理甚至修改它。
- 函数最后把 `String` 还给调用方。

如果只是只读分析，更常见的签名其实是：

```rust
fn analyze(text: &str) -> (usize, usize)
```

本题故意使用 `String`，目的是练习所有权返回。

### 元组元素从左到右求值

下面代码会出错：

```rust
(text, text.len(), text.chars().count())
```

因为第一个元素已经移动了 `text`，后面的 `text.len()` 和 `text.chars()` 不能再借用它。

正确方式是先计算，再移动：

```rust
let bytes = text.len();
let chars = text.chars().count();
(text, bytes, chars)
```

### `clone` 不是所有权问题的默认答案

`clone` 会复制堆数据。初学时遇到所有权错误，很容易用 `clone` 绕过去，但这会掩盖真正的问题。

优先考虑：

- 函数是否只需要借用？
- 是否可以先计算，再移动？
- 是否应该返回所有权？

## 常见坑

- 在元组第一个位置移动 `text` 后又继续使用。
- 为了通过编译滥用 `clone`。
- 忘记 `chars().count()` 是遍历计算，不是 O(1) 字段读取。

## 回看问题

- 什么时候函数应该接收 `String`，什么时候接收 `&str`？
- 为什么要先算 `bytes` 和 `chars`，再返回 `text`？
- `clone` 解决了什么，又可能掩盖什么？

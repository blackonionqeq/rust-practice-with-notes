# 25 API Design - 补充笔记

## 本题覆盖

- API 签名表达所有权。
- `impl Into<String>`。
- `&T` / `&mut T` / `T`。
- 返回切片 `&[T]`。

## 需要补充

### 参数类型表达调用方成本

```rust
fn make_report(title: impl Into<String>) -> Report
```

调用方可以传：

- `&str`
- `String`

函数内部再统一转成 `String`。

### 读、改、消费用不同签名

只读：

```rust
fn has_tag(report: &Report, tag: &str) -> bool
```

修改：

```rust
fn add_tag(report: &mut Report, tag: impl Into<String>)
```

消费：

```rust
fn into_title(report: Report) -> String
```

签名本身就告诉调用方这个函数会不会拿走所有权。

### 返回切片隐藏内部容器

```rust
fn tags(report: &Report) -> &[String]
```

比 `&Vec<String>` 更灵活。调用方只需要读一段连续数据，不需要知道内部一定是 `Vec`。

如果未来内部改成别的结构，公开 API 的压力更小。

### 少 clone 来逼自己设计 API

很多 clone 可以通过签名设计避免：

- 需要拥有：接收 `String` 或 `impl Into<String>`。
- 只需要读：接收 `&str` 或 `&T`。
- 需要修改：接收 `&mut T`。
- 需要交出内部值：消费 `self` 或参数。

## 常见坑

- 所有字符串参数都写 `String`，导致调用方被迫分配。
- 返回 `&Vec<T>`，暴露了不必要的容器细节。
- 只读函数误用 `&mut T`。
- 本应消费的函数却 clone 内部字段返回。

## 回看问题

- `impl Into<String>` 适合什么场景？
- `&T`、`&mut T`、`T` 分别表达什么所有权关系？
- 为什么返回 `&[String]` 比 `&Vec<String>` 更好？

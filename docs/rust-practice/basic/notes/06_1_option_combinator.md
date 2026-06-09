# 06.1 Option Combinator - 补充笔记

## 本题覆盖

- `Result<T, E>` 转 `Option<T>`。
- `Option::map`。
- 方法链写法。
- 闭包的最小用法。

## 需要补充

### `.parse()` 返回的是 `Result`

```rust
s.parse::<i32>()
```

返回类型是：

```rust
Result<i32, std::num::ParseIntError>
```

解析成功是 `Ok(number)`，失败是 `Err(error)`。

### `.ok()` 可以丢弃错误细节

```rust
s.parse::<i32>().ok()
```

会把：

- `Ok(value)` 转成 `Some(value)`
- `Err(error)` 转成 `None`

适合“不关心失败原因，只关心有没有值”的场景。

如果需要保留错误原因，就不应该用 `.ok()`，而应该继续使用 `Result`。

### `.map()` 只处理 `Some`

```rust
option.map(|n| n * 2)
```

含义是：

- 如果是 `Some(n)`，返回 `Some(n * 2)`。
- 如果是 `None`，保持 `None`。

所以完整写法：

```rust
s.parse::<i32>().ok().map(|n| n * 2)
```

表达的是：先尝试解析，成功就翻倍，失败就保持无值。

### 组合器适合简单转换

`match` 更直观，组合器更紧凑。

简单转换适合组合器：

```rust
parse().ok().map(...)
```

复杂分支、需要多步副作用或日志时，`match` 通常更清楚。

## 常见坑

- 以为 `.ok()` 是“检查成功”，其实它会丢弃错误。
- 在 `.map()` 里返回 `Some(n * 2)`，导致得到 `Option<Option<i32>>`。
- 为了短而短，把复杂逻辑硬塞进方法链。

## 回看问题

- `.ok()` 会丢掉什么信息？
- `Option::map` 遇到 `None` 会做什么？
- 什么情况下 `match` 比组合器更合适？

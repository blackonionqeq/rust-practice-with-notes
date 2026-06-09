# 12.1 Result Question Mark - 补充笔记

## 本题覆盖

- `?` 运算符。
- 多步 `Result` 串联。
- 提前返回错误。
- 手写错误转换和错误传播的区别。

## 需要补充

### `?` 的含义

在返回 `Result` 的函数里：

```rust
let first = parse_number(a)?;
```

大致等价于：

```rust
let first = match parse_number(a) {
    Ok(value) => value,
    Err(error) => return Err(error),
};
```

成功就取出里面的值；失败就从当前函数提前返回。

### 使用 `?` 的函数也必须返回兼容的错误类型

`parse_and_divide` 返回：

```rust
Result<i32, String>
```

它调用的 `parse_number` 和 `divide` 也都返回 `Result<i32, String>`，所以 `?` 可以直接传播 `String` 错误。

如果错误类型不同，需要用 `map_err`、`From` 或自定义错误类型转换。本题暂时不引入。

### `?` 不是只能用在最后一步

`?` 常用于把多个可能失败的步骤写成直线流程：

```rust
let first = parse_number(a)?;
let second = parse_number(b)?;
let result = divide(first, second)?;
Ok(result)
```

读起来像普通代码，但每一步都可能提前返回错误。

### `Ok(result)` 仍然需要写

`?` 只负责处理中间的 `Result`。函数最终成功时仍要返回 `Ok(result)`。

## 常见坑

- 在不返回 `Result` 的函数里直接用 `?`。
- 被 `?` 的函数错误类型和当前函数错误类型不兼容。
- 忘记最后用 `Ok(result)` 包起来。

## 回看问题

- `?` 展开后大概是什么样？
- 为什么 `print_result` 里不能直接用 `?`？
- 多个步骤都可能失败时，`?` 比嵌套 `match` 好在哪里？

# 12 Result Parse - 补充笔记

## 本题覆盖

- `Result<T, E>`。
- `Ok` 和 `Err`。
- `parse::<u32>()`。
- 用 `match` 处理错误。
- 避免 `unwrap`。

## 需要补充

### `Result` 表示可能失败的计算

```rust
Result<u32, String>
```

含义是：

- 成功时返回 `u32`。
- 失败时返回 `String` 错误信息。

相比 panic，`Result` 把错误作为普通返回值交给调用方处理。

### `parse` 的错误类型可以转换

`input.parse::<u32>()` 返回的错误不是 `String`，而是标准库里的解析错误类型。

本题要求统一返回：

```rust
Err(String::from("invalid age"))
```

所以用 `match` 把标准解析错误转换成自己的错误消息。

### `unwrap` 会在错误时 panic

```rust
let age = input.parse::<u32>().unwrap();
```

如果输入是 `"abc"`，程序会 panic。练习错误处理时，应显式处理 `Err`。

### 错误消息所有权

`String::from("invalid age")` 创建一个拥有所有权的错误消息。因为函数返回 `Result<u32, String>`，错误分支需要交出一个 `String`。

## 常见坑

- 忘记写 `parse::<u32>()`，编译器推断不出目标数字类型。
- `Err("invalid age")` 类型是 `&str`，和 `Result<u32, String>` 不匹配。
- 在 `print_age` 里用 `unwrap`，绕过了错误分支。

## 回看问题

- `Result<T, E>` 的两个类型参数分别表示什么？
- 为什么 `Err("invalid age")` 不等于 `Err(String::from("invalid age"))`？
- 什么时候应该返回错误，什么时候才适合 panic？

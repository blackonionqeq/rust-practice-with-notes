# 18 Error From - 补充笔记

## 本题覆盖

- 自定义错误枚举。
- 手写 `Display`。
- `From` 错误转换。
- `?` 自动转换错误类型。

## 需要补充

### 错误枚举可以归一化多种失败

```rust
enum AppError {
    Parse(std::num::ParseIntError),
    Zero,
}
```

这个类型把“解析失败”和“除零”统一成一个错误类型。函数签名就可以统一写：

```rust
Result<i32, AppError>
```

### `Debug` 给开发者，`Display` 给用户

`#[derive(Debug)]` 方便调试打印。

`Display` 控制 `{}` 的输出，通常面向用户：

```rust
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Parse(_) => write!(f, "invalid number"),
            AppError::Zero => write!(f, "cannot divide by zero"),
        }
    }
}
```

### `From` 让 `?` 能转换错误

`input.parse::<i32>()` 的错误类型是 `ParseIntError`。

如果当前函数返回 `Result<i32, AppError>`，那么 `?` 需要知道如何把 `ParseIntError` 变成 `AppError`。

实现：

```rust
impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::Parse(error)
    }
}
```

之后就可以：

```rust
let value = input.parse::<i32>()?;
```

### 保存原始错误有价值

虽然 `Display` 只打印 `invalid number`，但 `AppError::Parse(error)` 仍保存了原始解析错误，调试或日志里可能有用。

## 常见坑

- 实现了 `Display`，但忘记实现 `From`，导致 `?` 不能转换错误。
- `fmt` 返回类型写错，应该是 `std::fmt::Result`。
- 在 `Display` 中用 `println!`，而不是 `write!(f, ...)`。
- 错误类型设计成 `String`，后续难以区分错误种类。

## 回看问题

- 为什么自定义错误常用 enum？
- `Display` 和 `Debug` 分别服务什么场景？
- `?` 在错误类型不同时依赖什么转换？

# 12.2 From Conversion - 补充笔记

## 本题覆盖

- 给自己的类型实现标准库 trait。
- `impl From<T> for U` 的读法。
- `From` 和 `Into` 的关系。
- 为后面的 `?` 错误转换做铺垫。

## 需要补充

### `impl Trait for Type` 仍然是同一个结构

之前写过：

```rust
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("...")
    }
}
```

这里的结构是：

```rust
impl Trait for Type
```

本题写：

```rust
impl From<u32> for Age {
    fn from(value: u32) -> Self {
        Age { years: value }
    }
}
```

结构没有变，只是 trait 变成了标准库里的 `From<u32>`。

### `From<T> for U` 的读法

```rust
impl From<u32> for Age
```

可以读成：

> 为 `Age` 实现“从 `u32` 转换而来”的能力。

`u32` 是来源类型，`Age` 是目标类型。

所以 `from` 的签名是：

```rust
fn from(value: u32) -> Age
```

在 `impl` 内部可以写成：

```rust
fn from(value: u32) -> Self
```

这里的 `Self` 就是 `Age`。

### 实现 `From` 后会自动得到 `Into`

如果实现了：

```rust
impl From<u32> for Age
```

就可以写：

```rust
let age = Age::from(18);
let age: Age = 18u32.into();
```

通常优先实现 `From`，调用处需要时再用 `.into()`。

### 和错误转换的关系

后面会看到：

```rust
impl From<std::num::ParseIntError> for AppError
```

它的结构和本题一样：

```rust
impl From<来源类型> for 目标类型
```

区别只是来源类型从 `u32` 变成了 `ParseIntError`，目标类型从 `Age` 变成了 `AppError`。

## 常见坑

- 把 `impl From<u32> for Age` 读反，以为是在给 `u32` 实现能力。
- `from` 返回 `u32`，但它应该返回 `Age`。
- `.into()` 无法推断目标类型时，需要写类型标注，例如 `let age: Age = 18u32.into();`。

## 回看问题

- `impl From<u32> for Age` 中，哪个是来源类型，哪个是目标类型？
- `Self` 在这个 `impl` 里代表什么？
- 为什么通常实现 `From`，而不是手写 `Into`？

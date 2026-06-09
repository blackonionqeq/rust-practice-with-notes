# 21 Generics Bounds - 补充笔记

## 本题覆盖

- 泛型函数。
- 泛型结构体。
- trait bound。
- `where` 子句。

## 需要补充

### 泛型让代码适配多种类型

```rust
fn larger<T>(left: T, right: T) -> T
```

`T` 表示某个具体类型。调用时，编译器会根据参数推断 `T` 是什么。

### trait bound 限制泛型能力

不是所有类型都能比较大小。要使用 `>=` 或 `>`，需要：

```rust
T: PartialOrd
```

不是所有类型都能格式化到字符串里。要用 `{}`，需要：

```rust
T: std::fmt::Display
```

### `where` 让签名更清楚

短约束可以写在泛型参数后：

```rust
fn describe<T: Display>(value: T)
```

复杂一点时，`where` 更易读：

```rust
fn describe_pair<T>(pair: &Pair<T>) -> String
where
    T: std::fmt::Display,
```

### 不加 `Copy` 时要注意所有权

本题不允许使用 `Copy` bound，所以 `larger(left, right) -> T` 必须通过移动返回其中一个值。

这意味着比较时要先借用，再移动返回：

```rust
if left >= right {
    left
} else {
    right
}
```

比较表达式会自动借用，不需要 clone。

## 常见坑

- 忘记 `PartialOrd`，却直接比较泛型值。
- 忘记 `Display`，却用 `{}` 格式化泛型值。
- 为了返回值加 `Clone` 或 `Copy`，但题目不需要。
- 把泛型当成动态类型；Rust 泛型默认是编译期单态化。

## 回看问题

- 泛型参数 `T` 代表什么？
- trait bound 是在约束类型，还是在创建类型？
- 为什么本题不需要 `clone`？

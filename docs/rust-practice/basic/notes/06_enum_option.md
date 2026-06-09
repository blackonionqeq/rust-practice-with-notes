# 06 Enum Option - 补充笔记

## 本题覆盖

- 定义枚举。
- 枚举变体携带数据。
- `Option<T>`。
- `match` 解构枚举。

## 需要补充

### enum 表示“几种可能之一”

```rust
enum LoginStatus {
    Guest,
    User(String),
}
```

一个 `LoginStatus` 值要么是 `Guest`，要么是 `User(String)`。`User` 变体内部还携带一个用户名。

### `Option<T>` 替代空值

Rust 没有普通意义上的 `null`。可能不存在的值用：

```rust
Option<T>
```

它只有两种情况：

- `Some(value)`：有值。
- `None`：无值。

这样编译器会强迫你处理“没有值”的情况。

### `match` 必须穷尽

匹配 `Option<String>` 时，必须处理 `Some(name)` 和 `None`：

```rust
match find_user(1) {
    Some(name) => println!("found user: {}", name),
    None => println!("user not found"),
}
```

这能避免忘记空值分支。

### 匹配会移动内部值

`Some(name)` 会把 `String` 移动到 `name`。如果只是想借用，可以匹配引用：

```rust
match &option {
    Some(name) => println!("{}", name),
    None => {}
}
```

本题直接消费返回值，所以移动没有问题。

## 常见坑

- 用 `unwrap` 跳过 `None` 情况。
- `match` 漏掉一个变体。
- 匹配 `User(name)` 后还想继续使用原来的 enum 值。

## 回看问题

- `Option<T>` 为什么比 `null` 更安全？
- `Some(name)` 中的 `name` 拿到的是所有权还是借用？
- 为什么 `match` 要求覆盖所有可能？

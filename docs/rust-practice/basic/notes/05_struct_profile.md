# 05 Struct Profile - 补充笔记

## 本题覆盖

- 定义结构体。
- 字段初始化简写。
- 字段访问。
- struct update syntax。
- `#[derive(Debug)]` 和 `{:?}`。

## 需要补充

### 结构体字段也有所有权

结构体拥有它的字段。字段类型是 `String` 时，字段本身也有所有权。

```rust
struct UserProfile {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

创建 `UserProfile` 后，`username` 和 `email` 的所有权在结构体里。

### 字段初始化简写

当变量名和字段名相同时：

```rust
fn build_user(username: String, email: String) -> UserProfile {
    UserProfile {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
```

`username` 等价于 `username: username`。

### struct update syntax 会移动未显式覆盖的字段

```rust
let user2 = UserProfile {
    email: String::from("second@example.com"),
    ..user1
};
```

这里 `email` 使用新值，其他字段从 `user1` 来。

因为 `username: String` 会被移动到 `user2`，所以之后不能再使用 `user1.username`。但 `sign_in_count: u64` 和 `active: bool` 是 `Copy` 类型，单独使用这些字段通常仍可行。

实践中，把 `..user1` 后的 `user1` 视为“部分不可再用”更稳妥。

### `Debug` 是给开发者看的格式

要用 `{:?}` 打印结构体，需要：

```rust
#[derive(Debug)]
```

普通用户可读的格式通常实现 `Display`，调试输出用 `Debug`。

## 常见坑

- 忘记 `#[derive(Debug)]` 就用 `{:?}`。
- `..user1` 后继续使用已经移动的 `String` 字段。
- 把字段初始化简写误解成特殊语法，其实只是同名时的简写。

## 回看问题

- 结构体字段什么时候会移动？
- `Debug` 和 `Display` 有什么区别？
- `..user1` 到底复制了哪些字段，移动了哪些字段？

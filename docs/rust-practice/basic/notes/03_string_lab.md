# 03 String Lab - 补充笔记

## 本题覆盖

- `&str` 字符串切片。
- UTF-8 字符串的字节长度和字符数量。
- `.chars()` 迭代字符。
- `collect::<String>()` 拼回字符串。

## 需要补充

### `String` 和 `&str`

- `String` 是拥有所有权的、可增长的字符串类型。
- `&str` 是借用的字符串切片，通常只读。

函数只需要读取文本时，优先接收 `&str`：

```rust
fn summarize(text: &str) -> (usize, usize, String)
```

这样调用方既可以传 `&String`，也可以传字符串字面量。

### `.len()` 返回字节数

Rust 字符串内部是 UTF-8。`text.len()` 返回字节数，不是“字符数”。

例如 `"Rust语言练习"`：

- ASCII 字母每个 1 字节。
- 中文字符通常每个 3 字节。
- 所以字节数和字符数不一定相同。

### 不要随意按字节切中文字符串

这种写法可能 panic：

```rust
let part = &text[0..3];
```

因为切片边界必须落在 UTF-8 字符边界上。更稳妥的方式是：

```rust
let preview: String = text.chars().take(3).collect();
```

### `collect` 需要目标类型

`collect()` 能收集成很多容器，所以经常需要让编译器知道目标类型：

```rust
let preview: String = text.chars().take(3).collect();
```

或：

```rust
let preview = text.chars().take(3).collect::<String>();
```

## 常见坑

- 把 `.len()` 当作字符数。
- 用字节下标截取 UTF-8 文本。
- 忘记给 `collect` 标明目标类型。

## 回看问题

- `String` 和 `&str` 分别适合什么场景？
- 为什么中文字符串的 `.len()` 大于字符数量？
- 为什么 `.chars().take(3)` 比 `&text[0..3]` 更适合预览文本？

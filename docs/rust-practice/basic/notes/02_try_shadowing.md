# 02 Try Shadowing - 补充笔记

## 本题覆盖

- `String` 的所有权移动。
- 函数接收所有权并返回所有权。
- 元组返回多个值。
- 元组解构。
- 变量遮蔽 shadowing。

## 需要补充

### `String` 默认会移动

`String` 管理堆内存，不实现 `Copy`。把它传给接收 `String` 的函数时，所有权移动到函数里：

```rust
let text = String::from("rust");
measure(text);
// text 之后不能再用
```

如果函数还要把字符串交还给调用者，可以返回它：

```rust
fn measure(text: String) -> (String, usize) {
    let len = text.len();
    (text, len)
}
```

### 元组常用于临时组合多个返回值

`(String, usize)` 表示返回两个值。调用方可以用解构接收：

```rust
let (text, len) = measure(text);
```

这里新的 `text` 可以遮蔽旧的 `text`，但它是一个新绑定，重新拥有返回的 `String`。

### shadowing 不是 `mut`

`mut` 是修改同一个变量绑定中的值。

shadowing 是创建一个新的同名变量：

```rust
let value = "42";
let value = value.parse::<i32>().unwrap();
```

shadowing 可以改变类型，`mut` 不可以。

## 常见坑

- 函数接收 `String` 后，调用方还想直接使用原变量。
- 忘记先计算长度，先移动 `text` 后又借用它。
- 把 shadowing 理解成修改原变量。

## 回看问题

- `String` 传参为什么会移动？
- `let (text, len) = ...` 里的 `text` 是旧变量还是新变量？
- shadowing 和 `mut` 的区别是什么？

# 12.3 Associated Type Parser - 补充笔记

## 本题覆盖

- trait 关联类型。
- `type Output;` 的含义。
- `Self::Output` 的读法。
- 关联类型和泛型参数的区别。

## 需要补充

### 关联类型是 trait 里的“占位类型”

```rust
trait Parser {
    type Output;

    fn parse(&self, input: &str) -> Self::Output;
}
```

`Output` 不是值，也不是字段，而是一个类型位置的占位。

每个实现者都要指定自己的 `Output` 到底是什么：

```rust
impl Parser for NumberParser {
    type Output = Option<i32>;

    fn parse(&self, input: &str) -> Self::Output {
        input.parse::<i32>().ok()
    }
}
```

### `Self::Output` 的读法

在 trait 里，`Self` 表示“当前实现这个 trait 的具体类型”。

所以：

```rust
Self::Output
```

可以读成：

> 当前实现类型选择的 Output 类型。

对 `NumberParser` 来说，`Self::Output` 是 `Option<i32>`。

对 `WordCountParser` 来说，`Self::Output` 是 `usize`。

### 关联类型适合“一个实现对应一种输出”

如果一个类型实现 `Parser` 时，只应该有一种固定输出类型，关联类型就很合适。

例如：

```rust
impl Parser for WordCountParser {
    type Output = usize;
}
```

这表示 `WordCountParser` 作为 `Parser` 使用时，输出类型就是 `usize`。

### 和 `From<T>` 的联系

`From<T>` 本身没有关联类型，但它也体现了 trait 中的类型参数：

```rust
impl From<u32> for Age
```

关联类型和泛型参数都能让 trait 表达“和类型有关的规则”。初学时先记住：

- `From<T>`：来源类型写在 trait 的泛型参数里。
- `Parser::Output`：输出类型写在实现者的关联类型里。

## 常见坑

- 忘记在 `impl Parser for NumberParser` 里写 `type Output = ...;`。
- 把 `Self::Output` 当成字段访问。
- 想在同一个类型上用不同 `Output` 重复实现同一个关联类型 trait。

## 回看问题

- `type Output;` 是值、字段，还是类型占位？
- `Self::Output` 在 `NumberParser` 里具体是什么？
- 什么时候关联类型比给 trait 加泛型参数更自然？

# 11 Trait Summary - 补充笔记

## 本题覆盖

- 定义 trait。
- 为不同结构体实现同一个 trait。
- 用 `&self` 方法读取字段。
- 多态行为的基础。

## 需要补充

### trait 定义“能做什么”

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

它不关心具体类型是谁，只要求实现者提供 `summarize` 方法。

### `impl Trait for Type`

```rust
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
```

`Article` 和 `Comment` 可以用不同方式实现同一个行为。

### `format!` 创建新的 `String`

`format!` 类似 `println!`，但它不会打印，而是返回一个新的 `String`。

因为 `summarize` 的返回类型是 `String`，所以 `format!` 很合适。

### trait 不是继承

Rust 没有传统 class 继承。trait 更像是一组能力约定：

- 类型可以实现多个 trait。
- trait 可以有默认方法。
- trait 可以作为泛型约束或 trait object 使用。

本题先只练“定义和实现”，不引入泛型和 `dyn Trait`。

## 常见坑

- 在 trait 声明的方法后面忘记分号。
- `impl Summary for Article` 写成 `impl Article for Summary`。
- `summarize(&self)` 中直接移动 `self.title`，导致借用的方法里尝试移出字段。

## 回看问题

- trait 解决了什么问题？
- `format!` 和 `println!` 有什么区别？
- 为什么 `summarize` 使用 `&self` 而不是 `self`？

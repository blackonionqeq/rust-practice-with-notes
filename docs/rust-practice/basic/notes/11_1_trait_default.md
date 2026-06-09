# 11.1 Trait Default - 补充笔记

## 本题覆盖

- trait 必须方法。
- trait 默认方法。
- 默认方法调用必需方法。
- 为某个类型覆盖默认方法。

## 需要补充

### trait 可以同时有要求和默认实现

```rust
trait Named {
    fn name(&self) -> &str;

    fn introduce(&self) -> String {
        format!("Hello, I am {}", self.name())
    }
}
```

`name` 没有方法体，所以实现者必须提供。

`introduce` 有方法体，所以实现者可以直接使用，也可以覆盖。

### 默认方法可以依赖必需方法

默认的 `introduce` 调用了 `self.name()`。这说明 trait 可以把公共逻辑写一次，把类型差异留给必需方法。

这和“抽象类里有抽象方法和普通方法”的用途有些相似，但 Rust 没有 class 继承。

### 返回 `&str` 避免不必要分配

`name(&self) -> &str` 表示返回一个借用切片，通常写：

```rust
fn name(&self) -> &str {
    &self.name
}
```

不需要返回新的 `String`，也不需要 `clone`。

### 覆盖默认方法

`Teacher` 可以自己写 `introduce`：

```rust
fn introduce(&self) -> String {
    format!("Hello, I teach {}. My name is {}", self.subject, self.name())
}
```

覆盖后，调用 `teacher.introduce()` 会使用 `Teacher` 的版本。

## 常见坑

- `Student` 也手写了 `introduce`，没有练到默认方法。
- `name` 返回 `String`，导致需要 clone 或移动字段。
- 在默认方法中直接访问不存在的字段，例如 `self.name`。trait 不知道实现类型有哪些字段，只能调用 trait 方法。

## 回看问题

- 必需方法和默认方法有什么区别？
- 为什么 trait 默认方法里不能直接访问结构体字段？
- `&self.name` 为什么能作为 `&str` 返回？

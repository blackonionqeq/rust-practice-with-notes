# 24 Unit Tests - 补充笔记

## 本题覆盖

- 单元测试。
- `#[cfg(test)]`。
- `#[test]`。
- `assert_eq!`。

## 需要补充

### 单元测试通常和被测代码放一起

常见写法：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_uppercase() {
        assert_eq!(normalize_word("Rust"), "rust");
    }
}
```

`use super::*;` 让测试模块能访问外层模块的函数。

### `#[cfg(test)]` 只在测试构建时启用

普通 `cargo build` 不会编译测试模块。

`cargo test` 会启用 `cfg(test)`，编译并运行测试。

### `assert_eq!` 会打印左右值

```rust
assert_eq!(actual, expected);
```

失败时会显示两边的值，比 `assert!(actual == expected)` 更适合比较结果。

### 测试应该覆盖边界

本题至少覆盖：

- 大小写。
- 标点。
- 多行文本。
- 空输入。

这些比只测 happy path 更有价值。

## 常见坑

- 忘记给测试函数加 `#[test]`。
- 测试模块没有放在 `#[cfg(test)]` 下。
- 只测试正常输入，不测试空输入。
- 测试里复制实现逻辑，而不是检查结果。

## 回看问题

- `#[cfg(test)]` 的作用是什么？
- 为什么 `assert_eq!` 通常比 `assert!(a == b)` 更适合测试？
- 一个好测试应该覆盖哪些边界？

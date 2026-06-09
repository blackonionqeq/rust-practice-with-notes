# 01 Display Alpha - 补充笔记

## 本题覆盖

- Cargo 项目基本结构。
- `main` 函数作为二进制程序入口。
- 可变变量 `let mut total = 0`。
- 字符范围 `'a'..='z'`。
- `for` 循环和 `println!` 宏。

## 需要补充

### `cargo new` 创建的是包

`cargo new practice/basic/01_display_alpha --name display_alpha` 时：

- 目录名可以带数字前缀，方便排序。
- package name 不能以数字开头，所以用 `--name display_alpha`。
- `src/main.rs` 表示这是一个 binary crate。

### 范围语法

- `a..b` 是左闭右开，不包含 `b`。
- `a..=b` 是闭区间，包含 `b`。
- 本题要打印 `a` 到 `z`，所以应使用 `'a'..='z'`。

### 变量默认不可变

Rust 变量默认不可变：

```rust
let total = 0;
total += 1; // 编译错误
```

需要修改时显式写 `mut`：

```rust
let mut total = 0;
total += 1;
```

### `println!` 是宏

`println!` 后面的 `!` 表示它是宏，不是普通函数。宏可以接收可变数量参数，并在编译期展开。

## 常见坑

- 写成 `'a'..'z'` 会漏掉 `z`。
- 忘记 `mut` 会导致无法累加。
- Cargo 包名如果写成 `01_display_alpha` 会报错。

## 回看问题

- `..` 和 `..=` 的区别是什么？
- 为什么 `total` 需要 `mut`？
- 为什么目录名和 package name 可以不一样？

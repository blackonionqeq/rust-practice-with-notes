# 14 Module Shop - 补充笔记

## 本题覆盖

- `mod` 定义模块。
- 嵌套模块。
- `pub` 可见性。
- 使用 `::` 路径访问模块成员。
- 在单个 `main.rs` 中组织模块。

## 需要补充

### 模块用于组织命名空间

```rust
mod shop {
    pub mod inventory {
        pub fn item_count() -> u32 {
            3
        }
    }
}
```

`shop` 和 `inventory` 都是命名空间。调用时用路径：

```rust
shop::inventory::item_count()
```

### Rust 默认私有

模块、函数、结构体字段等默认都是私有的。

如果外部要访问 `inventory`，它必须是：

```rust
pub mod inventory
```

如果外部要调用函数，函数也必须是：

```rust
pub fn item_count() -> u32
```

只把函数写成 `pub` 不够；父模块路径上的可见性也要能走通。

### 当前练习不使用 `use`

不用 `use` 时，调用路径会比较完整：

```rust
shop::inventory::print_item("book");
```

以后可以用 `use` 把路径引入作用域：

```rust
use shop::inventory::print_item;
```

但本题先练习完整路径和 `::`。

### `mod` 不一定等于新文件

本题所有内容都写在 `src/main.rs`：

```rust
mod shop {
    // ...
}
```

后续项目变大后，可以把模块拆到文件中，例如 `src/shop.rs` 或 `src/shop/inventory.rs`。这属于下一阶段内容。

### crate、package、module 的粗略区别

- package：Cargo 管理的项目单元，有 `Cargo.toml`。
- crate：Rust 编译单元，分 binary crate 和 library crate。
- module：crate 内部组织代码的命名空间。

本题项目是一个 package，里面有一个 binary crate，crate 内部定义了 `shop` 和 `inventory` 模块。

## 常见坑

- 只写 `mod inventory`，外部访问时报私有错误。
- 函数忘记 `pub`。
- 把 `shop.inventory.item_count()` 写成点语法；模块路径要用 `::`。
- 以为 `mod shop` 必须创建新文件。

## 回看问题

- Rust 默认是公开还是私有？
- 为什么父模块和函数都可能需要 `pub`？
- `package`、`crate`、`module` 各自是什么层级？

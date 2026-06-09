# 15 Vec HashMap Inventory - 补充笔记

## 本题覆盖

- `Vec<T>` 动态数组。
- `HashMap<K, V>` 键值统计。
- 借用切片 `&[String]`。
- `entry(...).or_insert(...)` 计数模式。

## 需要补充

### `Vec<T>` 适合长度可变的同类型数据

数组 `[T; N]` 的长度是类型的一部分，`Vec<T>` 的长度可以运行时变化。

常见创建方式：

```rust
let mut items = Vec::new();
items.push(String::from("book"));
```

或：

```rust
let items = vec![String::from("book"), String::from("pen")];
```

### 函数参数优先接收切片

本题要求：

```rust
fn count_items(items: &[String]) -> HashMap<String, u32>
```

`&[String]` 比 `&Vec<String>` 更通用，因为它可以接收：

- `Vec<String>` 的切片。
- 数组切片。
- 其他连续序列的切片。

### `HashMap` 的 key 需要拥有数据

`HashMap<String, u32>` 拥有 key。遍历 `&[String]` 时拿到的是 `&String`，如果要把它作为 key 插入 map，通常需要克隆一份 key：

```rust
let count = counts.entry(item.clone()).or_insert(0);
*count += 1;
```

这里 clone 是合理的，因为 map 必须拥有自己的 key。

### `entry` 是计数常用写法

```rust
counts.entry(key).or_insert(0)
```

含义是：

- 如果 key 已存在，返回对应 value 的可变引用。
- 如果 key 不存在，插入默认值 `0`，再返回可变引用。

所以可以直接：

```rust
*counts.entry(item.clone()).or_insert(0) += 1;
```

初学时拆开写更清楚。

## 常见坑

- `count_items` 接收 `Vec<String>` 导致调用后 `items` 被移动。
- 用 `&Vec<String>`，错过更通用的切片参数。
- 忘记 `use std::collections::HashMap;`。
- 对 `or_insert` 返回的 `&mut u32` 忘记解引用。

## 回看问题

- `Vec<T>` 和数组的核心区别是什么？
- 为什么参数用 `&[String]` 比 `&Vec<String>` 更好？
- `HashMap<String, u32>` 为什么需要拥有 key？

# 08 Flow Counter - 补充笔记

## 本题覆盖

- `if` / `else if` / `else`。
- `while` 循环。
- `for` 循环。
- `break` 和 `continue`。
- 闭区间 `1..=10`。

## 需要补充

### `if` 是表达式

Rust 中 `if` 可以产生值：

```rust
let label = if n > 0 { "positive" } else { "not positive" };
```

但所有分支返回类型必须一致。本题只要求打印，所以不需要利用表达式特性。

### `while` 适合条件驱动循环

倒计时这种“只要条件还成立就继续”的逻辑适合 `while`：

```rust
while current > 0 {
    println!("{}", current);
    current -= 1;
}
```

要确保循环变量会变化，否则可能死循环。

### `for` 适合遍历范围或集合

```rust
for n in 1..=10 {
    // ...
}
```

相比手动维护下标，`for` 更不容易越界。

### `continue` 和 `break`

- `continue`：跳过本轮剩余代码，进入下一轮。
- `break`：结束整个循环。

本题跳过偶数，应使用 `continue`。

## 常见坑

- `while` 中忘记递减，造成死循环。
- 把 `1..10` 当成包含 10。
- `continue` 后面的代码在当前轮不会执行。

## 回看问题

- `1..10` 和 `1..=10` 有什么区别？
- 什么时候用 `while`，什么时候用 `for`？
- `break` 和 `continue` 的控制范围有什么不同？

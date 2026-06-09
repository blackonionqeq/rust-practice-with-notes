# 28 Rc Shared Tags - 补充笔记

## 本题覆盖

- `Rc<T>`。
- 单线程共享所有权。
- `Rc::clone`。
- 引用计数。

## 需要补充

### `Rc<T>` 允许多个所有者

`Rc` 是 reference counted pointer。多个地方可以共同拥有同一个值。

当强引用计数变成 0 时，值会被释放。

### `Rc::clone` 不克隆内部数据

```rust
let tag2 = Rc::clone(&tag);
```

这只增加引用计数，不会复制内部的 `String`。

为了表达意图，Rust 代码里通常优先写 `Rc::clone(&tag)`，而不是 `tag.clone()`。

### `Rc<T>` 只能用于单线程

`Rc<T>` 不是线程安全的。如果要跨线程共享所有权，使用 `Arc<T>`。

本题是单线程共享 tag，所以 `Rc<String>` 合适。

### `Rc<T>` 默认只提供共享不可变访问

`Rc<String>` 让多个文章共享同一个字符串，但不能直接修改里面的字符串。

如果需要共享且可变，通常会组合：

```rust
Rc<RefCell<T>>
```

这会引入运行时借用检查，应该谨慎使用。

## 常见坑

- 用 `tag.as_ref().clone()` 克隆了内部 `String`。
- 在多线程场景使用 `Rc`。
- 以为 `Rc` 自动解决可变性问题。
- 只看 `strong_count`，忘记 weak 引用不拥有值。

## 回看问题

- `Rc::clone` 和 `String::clone` 成本有什么区别？
- 为什么 `Rc` 不适合多线程？
- `Rc<T>` 解决的是共享所有权，还是共享可变性？

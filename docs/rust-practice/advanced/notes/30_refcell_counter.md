# 30 RefCell Counter - 补充笔记

## 本题覆盖

- `RefCell<T>`。
- 内部可变性。
- 运行时借用检查。
- `borrow` / `borrow_mut`。

## 需要补充

### 内部可变性允许 `&self` 中修改内部值

普通结构体方法要修改字段，通常需要：

```rust
fn increment(&mut self)
```

使用 `RefCell` 后，可以写：

```rust
fn increment(&self) {
    *self.value.borrow_mut() += 1;
}
```

外层是不可变借用，内部通过 `RefCell` 做运行时可变借用。

### 借用规则没有消失

`RefCell` 只是把借用检查从编译期推迟到运行时：

- 可以有多个不可变借用。
- 或者一个可变借用。
- 两者不能同时存在。

违反规则会 panic。

### `borrow_mut` 返回智能借用

```rust
let mut value = self.value.borrow_mut();
*value += 1;
```

`value` 离开作用域时，可变借用才结束。

所以要避免长时间持有 `borrow_mut()`。

### `Cell` 和 `RefCell` 的粗略区别

- `Cell<T>` 适合小的 `Copy` 值，用 get/set。
- `RefCell<T>` 适合非 `Copy` 或需要借用内部值的场景。

本题指定 `RefCell`，重点练习运行时借用。

## 常见坑

- 以为 `RefCell` 绕过了 Rust 借用规则。
- 同时持有 `borrow()` 和 `borrow_mut()` 导致 panic。
- 让 `borrow_mut()` 的作用域过长。
- 明明可以用普通 `&mut self`，却过早使用 `RefCell`。

## 回看问题

- 内部可变性解决了什么问题？
- `RefCell` 的借用错误发生在编译期还是运行时？
- 为什么要缩短 `borrow_mut()` 的作用域？

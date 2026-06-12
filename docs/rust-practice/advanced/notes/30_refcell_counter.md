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

## 补充说明：Cell、RefCell、Ref、RefMut

### `Copy` 和 `Cell` 解决的问题不同

实现了 `Copy` 的值确实可以直接复制，比如 `usize`、`bool`、`i32`。

但 `Copy` 只解决“值怎么复制”，不解决“只有 `&self` 时能不能修改字段”。

普通字段在 `&self` 方法里不能修改：

```rust
struct Counter {
    n: usize,
}

impl Counter {
    fn inc(&self) {
        // self.n += 1; // 不行：&self 不能修改字段
    }
}
```

`Cell<T>` 解决的是内部可变性：

```rust
use std::cell::Cell;

struct Counter {
    n: Cell<usize>,
}

impl Counter {
    fn inc(&self) {
        self.n.set(self.n.get() + 1);
    }

    fn current(&self) -> usize {
        self.n.get()
    }
}
```

这里 `get()` 把 `usize` 复制出来，`set()` 把新值写回去。

所以：

```text
Copy 解决：值能不能被复制
Cell 解决：只有 &self 时，内部小值能不能被修改
```

`Cell<T>` 不会把内部值借成 `&T` 或 `&mut T`，而是偏向“整体取出、整体放回”。因此它适合小的 `Copy` 值，比如计数器、开关、状态枚举。

### `RefCell` 适合需要借用内部值的场景

`RefCell<T>` 可以把内部值借出来：

```rust
use std::cell::RefCell;

let nums = RefCell::new(vec![1, 2, 3]);

nums.borrow_mut().push(4);

println!("{:?}", nums.borrow());
```

它适合 `Vec<T>`、`String`、`HashMap<K, V>`、自定义结构体等不适合用 `Cell` 整体 get/set 的数据。

### `Ref<T>` 和 `RefMut<T>` 是运行时借用守卫

`borrow()` 返回的不是普通 `&T`，而是 `Ref<T>`。

`borrow_mut()` 返回的不是普通 `&mut T`，而是 `RefMut<T>`。

对应关系：

```text
RefCell<T>::borrow()     -> Ref<T>     类似 &T
RefCell<T>::borrow_mut() -> RefMut<T>  类似 &mut T
```

它们像引用一样使用，是因为实现了 `Deref` / `DerefMut`：

```rust
use std::cell::RefCell;

let value = RefCell::new(10);

let r = value.borrow();
println!("{}", *r);
```

`Ref` / `RefMut` 的重点是：它们活着的时候，`RefCell` 会记录当前借用状态；它们被 drop 时，运行时借用才结束。

例如：

```rust
use std::cell::RefCell;

let nums = RefCell::new(vec![1, 2, 3]);

let r = nums.borrow();
// nums.borrow_mut(); // 会 panic：r 还活着，已有不可变借用

drop(r);

nums.borrow_mut().push(4);
```

记忆方式：

```text
Ref<T>    = RefCell 借出的只读访问令牌
RefMut<T> = RefCell 借出的可写访问令牌
drop 时释放运行时借用状态
```

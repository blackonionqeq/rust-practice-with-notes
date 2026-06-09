# 29 Weak Parent Tree - 补充笔记

## 本题覆盖

- `Weak<T>`。
- 避免 `Rc` 循环引用。
- `Rc::downgrade`。
- `Weak::upgrade`。
- `Rc<RefCell<T>>` 风格的内部可变性边界。

## 需要补充

### 父子关系容易形成循环引用

如果父节点强拥有子节点，子节点又强拥有父节点：

```text
parent -> child
child -> parent
```

两个 `Rc` 的强引用计数都不会归零，内存无法释放。

### 父引用应该用 `Weak`

父节点拥有子节点是强关系：

```rust
children: RefCell<Vec<Rc<Node>>>
```

子节点指回父节点只是观察关系：

```rust
parent: RefCell<Weak<Node>>
```

`Weak` 不增加 strong count，所以不会阻止值释放。

### `upgrade` 返回 `Option<Rc<T>>`

弱引用指向的值可能已经被释放，所以：

```rust
weak.upgrade()
```

返回：

```rust
Option<Rc<T>>
```

必须处理 `None`。

### `RefCell` 只放在需要变的字段上

本题 `name` 不需要变，所以是普通 `String`。

`parent` 和 `children` 需要在 `Rc<Node>` 共享后继续修改，所以用 `RefCell`。

## 常见坑

- parent 使用 `Rc<Node>`，导致循环引用。
- 忘记 `Rc::downgrade(parent)`。
- 对 `Weak` 直接当作有效父节点使用，不处理 `upgrade` 的 `None`。
- 把整个 `Node` 包成 `RefCell`，让可变范围过大。

## 回看问题

- `Weak` 为什么能避免循环引用？
- `upgrade` 为什么返回 `Option`？
- 为什么只给部分字段加 `RefCell`？

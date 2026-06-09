# 27 Box Recursive List - 补充笔记

## 本题覆盖

- `Box<T>`。
- 递归类型。
- 堆分配。
- 借用递归结构。

## 需要补充

### 递归 enum 需要间接层

下面写法无法编译：

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

因为 `List` 里面直接包含另一个 `List`，编译器无法计算它的大小。

使用 `Box<List>` 后，`Cons` 里保存的是固定大小的指针：

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

### `Box<T>` 拥有堆上的值

`Box<T>` 是拥有所有权的智能指针。离开作用域时，box 和它指向的值都会被释放。

它常用于：

- 递归类型。
- 把大对象放到堆上。
- trait object，例如 `Box<dyn Trait>`。

### 遍历时优先借用

`len(&list)`、`sum(&list)`、`to_vec(&list)` 只需要读，不应该消费列表。

匹配借用时，可以写：

```rust
match list {
    List::Cons(value, next) => ...
    List::Nil => ...
}
```

在匹配 `&List` 时，`value` 和 `next` 会是借用形式。

## 常见坑

- 递归 enum 忘记加 `Box`。
- 统计长度时把 list 移动掉。
- 为了遍历使用 `clone`。
- 把 `Box` 理解成共享所有权；`Box` 是单一所有权。

## 回看问题

- 为什么递归类型需要 `Box`？
- `Box<T>` 是否允许多个所有者？
- 哪些函数应该消费列表，哪些函数应该借用列表？

# 22 Trait Objects - 补充笔记

## 本题覆盖

- trait object。
- `dyn Trait`。
- `Box<dyn Trait>`。
- 动态分发。

## 需要补充

### trait object 用于“不同具体类型，同一行为”

`Text` 和 `Button` 是不同类型，不能直接放进同一个 `Vec<Text>`。

如果它们都实现了 `Render`，可以放进：

```rust
Vec<Box<dyn Render>>
```

这表示：一组拥有 `Render` 行为的值。

### 为什么需要 `Box`

`dyn Render` 是动态大小类型，编译期不知道具体大小。

`Box<dyn Render>` 是固定大小的指针，指向堆上的具体值，所以可以放进 `Vec`。

### 动态分发和泛型不同

泛型通常是静态分发：编译期知道具体类型。

trait object 是动态分发：运行时通过 vtable 找到具体实现。

适合场景：

- 一个集合里要放多种具体类型。
- 运行时才决定具体实现。

不适合场景：

- 性能极端敏感且类型在编译期已知。
- trait 不满足对象安全要求。

### 对象安全是 trait object 的前提

简单理解：trait 方法不能要求返回 `Self`，也不能有不适合放进 vtable 的泛型方法。

本题的：

```rust
fn render(&self) -> String;
```

是对象安全的。

## 常见坑

- 写成 `Vec<dyn Render>`，但 `dyn Render` 大小未知。
- 忘记 `dyn`。
- 把 trait object 和泛型混为一谈。
- trait 方法返回 `Self` 后尝试做 trait object。

## 回看问题

- 为什么 `Vec<Box<dyn Render>>` 可以放不同类型？
- `Box` 在这里解决了什么问题？
- 动态分发和泛型静态分发有什么区别？

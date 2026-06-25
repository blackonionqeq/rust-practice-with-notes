# 79 Pin Concept - 笔记

## 本题会用到的前置知识点

- `Box<T>` 与堆分配。
- 移动导致裸指针失效（78 题）。

## 核心结论

- `Pin<P>` 的承诺：**被 pin 的值不会再被移动**（地址固定）。
- `PhantomPinned`：让类型实现 `!Unpin`，从而让 `Pin` 的约束生效。
- `Pin` 本身不阻止代码移动值；它通过类型系统让"试图移动"无法通过编译。
- 在堆上 pin：`Box::pin(value)` → `Pin<Box<T>>`，地址固定在堆上。

## 需要补充看懂的知识

### `Pin<P>` 固定的是指针背后的值，不是变量名

`let pinned: Pin<Box<Unmovable>>` 这个变量本身仍然可以移动到另一个绑定里，因为移动的是 `Pin<Box<_>>` 这个指针包装和值为堆地址的 `Box` 元数据。

真正不能被移动的是 heap allocation 里的 `Unmovable`。换句话说，`Pin<Box<T>>` 可以换变量名，不能把里面的 `T` 从固定地址搬出来。

### `PhantomPinned` 让类型自动变成 `!Unpin`

大多数类型默认是 `Unpin`，表示即使被 pin 了也可以安全移动。加上 `PhantomPinned` 后，这个类型就不再自动实现 `Unpin`。

只有 `T: !Unpin` 时，`Pin<P>` 的"不能移动内部值"约束才真正有意义。否则 `Pin<Box<i32>>` 这类类型几乎只是普通 `Box<i32>`。

### `Pin` 不会帮你创建自引用字段

`Box::pin` 只负责先把值放到稳定地址上，再返回 `Pin<Box<T>>`。如果你要在值内部保存指向自身字段的裸指针，还需要额外初始化步骤，并且要维护一整套 unsafe invariant。

本题只学概念：固定地址和 `!Unpin` 的关系，不要求实现真正自引用。

## 核心示例

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;

struct Unmovable {
    data: String,
    _pin: PhantomPinned,  // 让此类型实现 !Unpin
}

fn main() {
    let pinned: Pin<Box<Unmovable>> = Box::pin(Unmovable {
        data: String::from("hello"),
        _pin: PhantomPinned,
    });

    println!("{}", pinned.data);  // 可以读取

    // let moved = *pinned;
    // ↑ 编译错误：cannot move out of `*pinned`
    // 因为 Unmovable: !Unpin，Pin 禁止通过解引用移走内部值
}
```

## `Pin` 防止什么

| 操作 | `T: Unpin` | `T: !Unpin` |
|---|---|---|
| 读取字段 | 允许 | 允许 |
| 获取 `&mut T` | 允许 | 需要 `unsafe` |
| 移出（`let x = *pin`）| 允许 | **禁止** |

## 常见坑

- 以为 `Pin` 会在运行时防止内存被复制——不是，`Pin` 是纯编译期约束。
- 以为任何类型放进 `Pin<Box<T>>` 都会被固定——只有 `!Unpin` 类型才受约束，`Unpin` 类型不受限（见 80 题）。
- `PhantomPinned` 是零大小类型，不占内存，只影响 trait 实现。

## 回看问题

- `PhantomPinned` 加在 struct 里之前和之后，`Pin<Box<T>>` 的行为有什么不同？
- 如果不用 `Box::pin` 而用 `Pin::new(Box::new(...))`，结果一样吗？

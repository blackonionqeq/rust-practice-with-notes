# 67 Derive vs Manual Impl - 笔记

## 本题会用到的前置知识点

- struct 定义与 impl 块。
- trait 实现方式。
- `fmt::Debug` 的签名。
- `PartialEq` 的签名。

## 核心结论

- `#[derive(Debug, PartialEq, ...)]` 是编译期代码生成：它展开成等价的 `impl` 块，运行时没有额外开销。
- 手写 impl 和 derive 版本在功能上等价，derive 只省掉了样板代码。
- derive 有条件限制：结构体每个字段必须也实现了对应的 trait，否则编译报错。

## 核心示例

### 手写版本

```rust
use std::fmt;

struct Point { x: f64, y: f64 }

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
```

### derive 等价版本

```rust
#[derive(Debug, PartialEq)]
struct Point { x: f64, y: f64 }
```

两版本行为相同。derive 省掉了 8 行样板 impl。

## 常见坑

- 误以为 derive 有运行时开销（它没有）。
- 字段类型不满足 trait bound 时 derive 会报错，而手写 impl 可以自定义逻辑。
- `PartialEq` 和 `Eq` 有区别：`f64` 实现了 `PartialEq` 但未实现 `Eq`（因为 NaN != NaN）。

## 回看问题

- 手写 `Debug` 和 derive 的 `Debug` 格式输出一样吗？
- 如果字段类型没有实现 `PartialEq`，derive 会怎样？
- 什么情况下需要手写 impl 而不能直接 derive？

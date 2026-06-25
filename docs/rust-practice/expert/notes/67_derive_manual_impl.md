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
- derive 生成的是“默认语义”：逐字段 `Debug`、逐字段比较等。
- 需要隐藏字段、改变比较规则、或处理浮点特殊语义时，就要考虑手写 impl。

## 需要补充看懂的知识

### derive 是编译期帮你写 impl

```rust
#[derive(Debug, PartialEq)]
struct Point { x: f64, y: f64 }
```

可以理解为编译器在编译期帮你补上类似代码：

```rust
impl std::fmt::Debug for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// 生成字段格式化逻辑
		f.debug_struct("Point")
			.field("x", &self.x)
			.field("y", &self.y)
			.finish()
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}
}
```

真实生成代码不必完全长这样，但行为上可以这样理解：derive 不是运行时反射，也不是运行时动态查找。

### 字段也必须实现对应 trait

derive 会递归依赖字段类型：

```rust
#[derive(Debug, PartialEq)]
struct User {
	name: String,
}
```

这能通过，因为 `String` 实现了 `Debug` 和 `PartialEq`。

如果字段类型没有实现对应 trait，derive 就会失败：

```rust
struct Secret;

// 错误思路：Secret 没有实现 Debug / PartialEq
// #[derive(Debug, PartialEq)]
// struct Wrapper {
// 	secret: Secret,
// }
```

这时可以给字段类型也实现/derive trait，或者手写外层 impl，跳过某些字段。

### derive 的语义是逐字段默认语义

`PartialEq` 的 derive 通常等价于所有字段都相等：

```rust
self.x == other.x && self.y == other.y
```

这对很多普通数据结构是正确的。但如果你的业务规则不是“所有字段都参与比较”，就不该直接 derive。

例如：

```rust
struct Session {
	id: u64,
	last_seen_ms: u64,
}
```

如果你只想按 `id` 判断是否同一个 session，手写 `PartialEq` 会比 derive 更准确。

### `PartialEq` 和 `Eq` 的差别

`PartialEq` 表示“可以比较相等”。`Eq` 表示这个相等关系满足更强的约定，尤其是自反性：一个值应该等于它自己。

`f64` 只有 `PartialEq`，没有 `Eq`，因为：

```rust
let x = f64::NAN;
assert!(x != x);
```

所以 `Point { x: f64, y: f64 }` 可以 derive `PartialEq`，但不能 derive `Eq`。

### `Debug` 输出不一定和手写完全一样

derive 的 `Debug` 通常输出结构化格式，例如：

```text
Point { x: 1.0, y: 2.0 }
```

如果你手写 `Debug`，可以选择不同格式、隐藏字段、改名或输出摘要。也就是说，derive 是默认实现，不是唯一正确格式。

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
- 以为 derive 的比较规则一定符合业务规则。
- 手写 `Debug` 后期待它和 derive 输出逐字符一致。

## 回看问题

- 手写 `Debug` 和 derive 的 `Debug` 格式输出一样吗？
- 如果字段类型没有实现 `PartialEq`，derive 会怎样？
- 什么情况下需要手写 impl 而不能直接 derive？
- 这个类型的“相等”是否真的等于所有字段都相等？

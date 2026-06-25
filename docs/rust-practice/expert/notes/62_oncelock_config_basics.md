# 62 OnceLock Config Basics - 笔记

## 本题会用到的前置知识点

- `HashMap` 基础。
- `Result` / `Option` 错误处理。
- `static` 的基础概念。

## 核心结论

- 全局状态优先做成“初始化一次 + 后续只读”。
- `OnceLock` 比 `static mut` 安全得多。
- 第二次初始化应明确返回错误，不要静默覆盖。
- `static` 全局值可能被多处代码同时访问，所以类型必须支持安全共享。
- `OnceLock<T>` 把“还没初始化”和“已经初始化”的状态封装起来，让读取方不用写 `unsafe`。

## 需要补充看懂的知识

### 为什么全局可变状态危险

全局变量没有普通局部变量那样清楚的调用边界。任何地方都可能读它，也可能尝试改它。如果再考虑多线程，同时读写会带来数据竞争。

这就是为什么不要直接使用：

```rust
// 本阶段不要用：全局可变状态需要 unsafe，且很容易破坏不变量
// static mut CONFIG: Option<HashMap<&'static str, &'static str>> = None;
```

第 62 题选择的模型更窄：

1. 程序启动时初始化一次。
2. 初始化后只读。
3. 第二次初始化明确报错。

这个模型容易推理，也更接近实际配置表、注册表这类场景。

### `OnceLock` 的状态模型

可以把 `OnceLock<T>` 简化理解成两个状态：

- empty：还没有值。
- initialized：已经有一个 `T`。

相关操作：

```rust
CONFIG.set(map)
```

如果之前是 empty，就存入 `map` 并返回 `Ok(())`。如果已经 initialized，就返回 `Err(map)`，表示这次传入的值没有被写进去。

```rust
CONFIG.get()
```

如果已经初始化，返回 `Some(&T)`；否则返回 `None`。

### 为什么 `static CONFIG` 可以安全读取

```rust
static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
```

`CONFIG` 是全局变量。全局变量可以从很多地方访问，所以它的内部同步必须由类型保证。`OnceLock` 来自 `std::sync`，它负责保证初始化过程只成功一次，读取时也不需要调用者写 `unsafe`。

本题里的 `HashMap` 初始化后不再修改，后续只通过共享引用读取：

```rust
CONFIG.get().and_then(|m| m.get(key).copied())
```

这里 `m.get(key)` 返回的是 `Option<&&'static str>`：先从 `HashMap` 里借到 value 的引用，而 value 本身也是 `&'static str`。`.copied()` 把它变成 `Option<&'static str>`，更适合作为函数返回值。

### 未初始化和 key 不存在要分清

`get_config` 的返回类型是：

```rust
fn get_config(key: &str) -> Option<&'static str>
```

它会把两种情况都表示成 `None`：

- `CONFIG` 还没有初始化。
- `CONFIG` 已初始化，但没有这个 key。

本题可以接受这个简化。真实项目里如果需要区分原因，可以返回 `Result<Option<&'static str>, Error>` 或定义更明确的错误类型。

### 为什么 key/value 用 `&'static str`

本题的全局 map 存在于 `static CONFIG` 中。为了让例子保持简单，key 和 value 都使用字符串字面量：

```rust
map.insert("mode", "dev");
```

这些字面量可以作为 `&'static str`。如果要把运行时读入的 `String` 放进全局配置，就需要重新设计所有权：例如让 map 存 `String`，或者在非常受控的场景下使用第 59 题的 `Box::leak`。不要随手把局部 `String` 的引用塞进全局变量。

## 核心示例

```rust
use std::collections::HashMap;
use std::sync::OnceLock;

static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn init_config() -> Result<(), &'static str> {
	let mut map = HashMap::new();
	map.insert("mode", "dev");
	map.insert("region", "local");
	CONFIG.set(map).map_err(|_| "config already initialized")
}

fn get_config(key: &str) -> Option<&'static str> {
	CONFIG.get().and_then(|m| m.get(key).copied())
}
```

理解重点：`set` 只会成功一次，读取走 `get`，全程不需要 `static mut`。

## 常见坑

- 继续使用 `static mut`。
- 不区分“未初始化/初始化失败/key 不存在”。
- 把过多业务逻辑塞进初始化函数。
- 忽略 `set` 的返回值，导致第二次初始化失败却没人知道。
- 把局部 `String` 的引用塞进全局配置。

## 回看问题

- 这份配置是否真的必须全局？
- 初始化后是否保持只读？
- 第二次初始化失败时，调用方能否看见错误？
- 未初始化和 key 不存在是否需要区分？

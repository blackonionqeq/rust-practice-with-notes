# 44 Documentation Tests - 补充笔记

## 背景知识

### 文档注释的三种写法

```rust
/// 单行文档注释：给紧跟着的 item（函数、结构体等）加文档
/// 支持多行，每行一个 ///
pub fn add(a: i32, b: i32) -> i32 { a + b }

/** 块文档注释：用 /** */ 包裹
    不太常用，但存在 */
pub fn sub(a: i32, b: i32) -> i32 { a - b }

//! 模块级文档：描述整个文件或模块
//! 放在文件最开头
```

`cargo doc` 会把这些注释渲染成 HTML 文档。而文档里的 **代码块**（用 `` ```rust `` 包裹）会被 `cargo test` 自动编译和运行——这就是 doc test。

### doc test 的基本形式

```rust
/// 计算两个数的和。
///
/// # Examples
///
/// ```
/// use my_crate::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

运行 `cargo test` 时，Rust 会：
1. 提取所有文档里的 ` ```rust ` 代码块。
2. 把每个代码块编译成一个小程序（自动引入当前 crate）。
3. 运行它——如果 panic 了，测试失败。

所以你的文档示例**永远保持可编译、可运行**——文档和测试合二为一。

### 代码块的标记

```rust
/// ```rust        ← 默认：编译 + 运行
/// assert_eq!(1 + 1, 2);
/// ```
///
/// ```no_run      ← 编译但不运行（适合有副作用的代码）
/// use my_crate::start_server;
/// start_server()?;  // 不会真的启动
/// ```
///
/// ```ignore      ← 完全忽略（既不编译也不运行）
/// // 伪代码，只是为了说明
/// let x = some_magic();
/// ```
///
/// ```compile_fail ← 期望编译失败（测试错误提示是否正确）
/// use my_crate::add;
/// add("not a number", 1);  // 应该编译不过
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

| 标记 | 编译？ | 运行？ | 用途 |
|------|--------|--------|------|
| ` ```rust ` | ✅ | ✅ | 标准用法 |
| ` ```no_run ` | ✅ | ❌ | 编译检查但不执行 |
| ` ```ignore ` | ❌ | ❌ | 纯展示 |
| ` ```compile_fail ` | ✅（期望失败） | ❌ | 验证错误类型 |

### 隐藏行（Hidden Lines）

有时你需要在示例里写 setup 代码，但不想在文档里展示。用 `#` 开头的行：

```rust
/// ```
/// # use my_crate::Config;   // ← 文档里看不到这行
/// # let config = Config::default();  // ← 这行也隐藏
/// assert_eq!(config.port, 8080);     // ← 只有这行显示
/// ```
pub struct Config {
    pub port: u16,
}
```

`cargo doc` 生成的 HTML 里，`#` 行被隐藏；`cargo test` 运行时会包含它们。

### doc test 里引入当前 crate

doc test 默认能 `use` 当前 crate 的公开项：

```rust
/// ```
/// use my_crate::add;  // 直接 use，不需要 extern crate
/// assert_eq!(add(3, 4), 7);
/// ```
```

但 doc test **不能**访问私有函数——和集成测试一样，只能用 `pub` API。

### 返回 Result 的 doc test

```rust
/// ```
/// use my_crate::parse;
/// let result: i32 = parse("42")?;  // 用 ? 传播错误
/// assert_eq!(result, 42);
/// ```
pub fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}
```

如果 doc test 函数返回 `Result`，`?` 就能用了。编译器会自动推断返回类型。

### 常用文档章节标题

`cargo doc` 会自动识别 Markdown 标题并排版：

```rust
/// 一句话描述这个函数。
///
/// # Examples      ← 自动出现在 "Examples" 折叠区
///
/// ```
/// ...
/// ```
///
/// # Errors        ← 描述可能的错误
///
/// 当输入为空字符串时返回 `ParseIntError`。
///
/// # Panics        ← 描述何时会 panic
///
/// 当 `divisor` 为 0 时 panic。
///
/// # Safety        ← unsafe 函数必须写
///
/// 调用者必须保证指针有效。
```

常用的章节标题：`Examples`、`Errors`、`Panics`、`Safety`、`Arguments`。

### 运行 doc test

```bash
# 运行所有测试（包括 doc test）
cargo test

# 只运行 doc test
cargo test --doc

# 查看生成的文档
cargo doc --open
```

### doc test vs 单元测试 vs 集成测试

| | doc test | 单元测试 | 集成测试 |
|--|---------|---------|---------|
| 位置 | 文档注释里 | `#[cfg(test)] mod` | `tests/` 目录 |
| 目的 | 保证示例可编译可运行 | 验证内部逻辑 | 验证公开 API |
| 谁看 | 用户（在 docs.rs 上） | 开发者 | 开发者 |
| 可见性 | 只有 `pub` | 包括私有 | 只有 `pub` |

最佳实践：**核心公开函数都应该有带 `# Examples` 的文档注释**。这样用户有示例可看，你也有回归测试。

---

## 本题覆盖

Practice writing examples that are checked by `cargo test`.

## 需要重点理解

- 文档里的 `` ```rust `` 代码块会被自动编译和运行。
- `#` 开头的行在文档里隐藏但在测试时执行。
- doc test 只能访问 `pub` API——和用户的使用方式完全一致。
- `# Examples` 是最受推荐的文档章节标题。

## 常见坑

- 文档示例忘了写 `use`，导致 `cargo test` 报 "found maybe unresolved"。
- 示例代码用了 `.unwrap()` 而不是 `?` 或 `assert!`，示例不够惯用。
- 该用 `no_run` 的地方没加（比如启动服务器），导致 `cargo test` 卡住。
- `#` 隐藏行和可见行混在一起，文档看起来不连贯。

## 回看问题

- 你的文档示例是否展示了最常见的用法？是否有不必要的细节？
- 如果示例编译失败，错误信息是否能帮用户理解为什么？
- 你有没有为可能 panic 或返回错误的函数写 `# Panics` / `# Errors`？

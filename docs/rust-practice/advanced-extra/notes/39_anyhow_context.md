# 39 Anyhow Context - 补充笔记

## 背景知识

### 上一题回顾：thiserror vs anyhow

- **thiserror**（38 题）：定义自己的错误枚举，适合**库**，让调用者能 `match` 不同的错误变体。
- **anyhow**（本题）：一个通用的错误包装器，适合**应用 / binary**，只关心"出了什么错"而不需要区分类型。

```rust
// 库：返回具体错误类型，调用者可以 match
fn parse_config(text: &str) -> Result<Config, AppError> { ... }

// 应用：用 anyhow 汇总所有错误，只管打印
fn main() -> anyhow::Result<()> {
    let config = parse_config(&text)?;  // AppError 自动转为 anyhow::Error
    Ok(())
}
```

### anyhow::Result 是什么

```rust
use anyhow::Result;

fn main() -> Result<()> {
    // 任何实现了 std::error::Error 的错误都能用 ? 传播到这里
    let content = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(())
}
```

`anyhow::Result<T>` 等价于 `Result<T, anyhow::Error>`。`anyhow::Error` 可以装任何实现了 `std::error::Error + Send + Sync + 'static` 的错误类型，所以你不需要为每种底层错误写 `From` impl——`?` 自动帮你转换。

### .context() —— 给错误加上下文

裸错误消息往往不够用。比如：

```
No such file or directory (os error 2)
```

哪个文件没找到？不知道。`.context()` 让你在错误上附加说明：

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config.json")
        .context("Failed to read config.json")?;
    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config.json")?;
    Ok(())
}
```

现在错误信息变成了：

```
Failed to parse config.json

Caused by:
    expected `,` at line 3 column 5
```

### .with_context() —— 惰性上下文

`.context()` 的参数是立刻求值的。如果上下文字符串很贵（比如格式化），但你只在出错时才需要，可以用 `.with_context()`：

```rust
let port = env::var("PORT")
    .with_context(|| format!("Failed to read PORT from environment (cwd: {:?})", env::current_dir().unwrap()))?;
```

闭包 `||` 只在出错时才执行。

### 错误链（Error Chain）

`anyhow` 会保留完整的错误链。每一层 `.context()` 都会追加到链条上：

```
Failed to start server

Caused by:
    0: Failed to read config.json
    1: No such file or directory (os error 2)
```

打印时用 `{:#}` 只打印最外层，用 `{:?}` 打印调试信息（包含完整链）：

```rust
if let Err(e) = run() {
    eprintln!("{e:#}");   // 只打印最外层：Failed to start server
    eprintln!("{e:?}");   // 完整链 + backtrace（如果有）
}
```

### Backtrace（回溯）

开启了 `RUST_BACKTRACE=1` 环境变量后，`anyhow` 会自动捕获错误发生处的调用栈：

```bash
RUST_BACKTRACE=1 cargo run
```

这比只用 `panic!` 的 backtrace 更有用，因为 `anyhow` 在错误发生时就捕获了栈，而不是在 panic 时。

### anyhow 的 .map_err() 替代方案

不用 anyhow 时，给错误加上下文只能用 `.map_err()`：

```rust
// 不用 anyhow：
let content = std::fs::read_to_string("config.json")
    .map_err(|e| format!("Failed to read config.json: {e}"))?;

// 用 anyhow：
let content = std::fs::read_to_string("config.json")
    .context("Failed to read config.json")?;
```

`.context()` 更简洁，而且保留了原始错误的类型信息（`source()` 还能用）。

### anyhow 和 thiserror 搭配使用

最佳实践是**库用 thiserror，应用用 anyhow**：

```
[dependencies]
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"        // 如果库也在同一个 crate 里
```

```rust
// 库代码：返回具体错误
fn parse_config(text: &str) -> Result<Config, AppError> { ... }

// 应用代码：用 anyhow 收集上下文
fn main() -> anyhow::Result<()> {
    let text = std::fs::read_to_string("config.json")
        .context("Cannot read config")?;
    let config = parse_config(&text)
        .context("Cannot parse config")?;   // AppError 被 ? 自动转为 anyhow::Error
    Ok(())
}
```

---

## 本题覆盖

Practice binary-level error handling with `anyhow` and contextual messages.

## 需要重点理解

- `anyhow::Result<T>` 可以装任何错误，不需要自定义错误类型。
- `.context()` 让错误消息对人类可读，而不是只有机器级别的错误码。
- 库返回具体类型（thiserror），应用汇总上下文（anyhow）——两者互补。
- 错误链帮你在调试时追踪"到底哪里出了问题"。

## 常见坑

- 在库里用 `anyhow`——调用者无法 `match` 不同错误，API 变得不透明。
- `.context()` 写的信息太笼统（比如 "something went wrong"），没说清楚到底哪一步出错。
- 忘记给每一步 IO / 解析操作加 `.context()`，导致错误信息链断裂。
- 在热路径（hot path）里用 `.context(format!(...))` 而不是 `.with_context(|| ...)`，每次都分配字符串。

## 回看问题

- 你的 `.context()` 信息是否足够让用户自己定位问题？
- 如果去掉 `anyhow`，手动用 `map_err` 写同样的逻辑，代码会多多少？
- 库和应用之间的错误边界在哪里？

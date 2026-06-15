# 38 Thiserror App Error - 补充笔记

## 背景知识

### Rust 的错误处理基础

Rust 没有 `try/catch`。函数通过返回 `Result<T, E>` 表示可能失败：

```rust
fn read_username() -> Result<String, std::io::Error> {
    std::fs::read_to_string("username.txt")
}
```

`E` 是错误类型。标准库提供了 `std::io::Error`、`std::fmt::Error` 等，但它们彼此不兼容——你没法让一个函数同时返回 `io::Error` 和 `fmt::Error`。

### 自定义错误类型

当你的库需要返回多种不同的错误时，需要定义自己的错误类型。最原始的写法是手动实现 `std::error::Error` trait：

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    NotFound(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {e}"),
            AppError::Parse(e) => write!(f, "Parse error: {e}"),
            AppError::NotFound(name) => write!(f, "'{name}' not found"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::Parse(e) => Some(e),
            AppError::NotFound(_) => None,
        }
    }
}

// 还要为每个被包装的错误写 From impl...
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
```

大量样板代码，每个变体都要写三遍（定义、Display、source）。`thiserror` 就是为了消除这些样板而生的。

### thiserror 基本用法

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("'{0}' not found")]
    NotFound(String),
}
```

一行 `#[derive(Error)]` + `#[error("...")]` 就替代了上面几十行手动代码。

关键属性解读：

| 属性 | 作用 | 示例 |
|------|------|------|
| `#[error("...")]` | 自动生成 `Display` 实现 | `#[error("file not found: {name}")]` |
| `#[from]` | 自动生成 `From` 实现 + 设置 `source()` | `Io(#[from] std::io::Error)` |
| `{0}` | 引用变体字段（位置参数） | `NotFound(String)` → `#[error("{0}")]` |
| `{name}` | 引用命名字段 | `#[error("bad port: {port}")] BadPort { port: u16 }` |

### `#[from]` 的魔法

```rust
#[error("IO error")]
Io(#[from] std::io::Error),
```

加了 `#[from]` 后，你可以直接用 `?` 把 `io::Error` 转成 `AppError`：

```rust
fn load_config() -> Result<Config, AppError> {
    let text = std::fs::read_to_string("config.toml")?; // io::Error 自动转为 AppError::Io
    // ...
    Ok(config)
}
```

没有 `#[from]`，你需要手写 `impl From<io::Error> for AppError { ... }`。

### 本题的 stream 读取版本

这题要求用 `BufReader` 按行读取，而不是一次性把整个文件读进内存。核心差异是：现在有两个地方可能产生 IO 错误：

- `File::open(path)?`：打开文件失败。
- `line?`：读取某一行时失败。

示例结构：

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_numbers(path: &Path) -> Result<Vec<i64>, AppError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let number = trimmed.parse::<i64>()?;
        numbers.push(number);
    }

    if numbers.is_empty() {
        return Err(AppError::InvalidInput("no numbers found".to_string()));
    }

    Ok(numbers)
}
```

如果使用一次性读取，也可以写成下面这样，但这不是本题要求的 streaming 版本：

```rust
// 也可用 read_to_string 版：写法更短，但会一次性把整个文件读进内存。
// fn read_numbers(path: &Path) -> Result<Vec<i64>, AppError> {
//     let text = std::fs::read_to_string(path)?;
//     let mut numbers = Vec::new();
//
//     for line in text.lines() {
//         let trimmed = line.trim();
//
//         if trimmed.is_empty() {
//             continue;
//         }
//
//         let number = trimmed.parse::<i64>()?;
//         numbers.push(number);
//     }
//
//     if numbers.is_empty() {
//         return Err(AppError::InvalidInput("no numbers found".to_string()));
//     }
//
//     Ok(numbers)
// }
```

注意这里的 `text.lines()` 和 `reader.lines()` 很容易混淆：

- `text.lines()` 是在已经加载到内存里的字符串上切行，迭代项是 `&str`。
- `reader.lines()` 是边读文件边产出行，迭代项是 `io::Result<String>`，所以循环里需要 `let line = line?;`。

### 结构体变体 vs 元组变体

```rust
#[derive(Error, Debug)]
pub enum AppError {
    // 元组变体：{0} 引用第一个字段
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // 结构体变体：{name} 引用命名字段
    #[error("invalid port {port} for host {host}")]
    BadAddress { host: String, port: u16 },
}
```

### 什么时候用 thiserror

- **写库（library）时**：调用者需要根据错误类型做不同处理（比如重试 IO、提示用户修正输入），`thiserror` 提供的类型信息很有用。
- **写应用（binary）时**：通常直接用 `anyhow`（下一题），不需要自定义错误类型。

简单判断：如果你的错误需要在 `match` 里区分不同情况，用 `thiserror`；如果只需要打印 / 记录日志，用 `anyhow`。

### thiserror vs 手动实现

| 手动实现 | thiserror |
|---------|-----------|
| 完全控制，灵活 | 惯用写法，简洁 |
| 大量样板代码 | 几行搞定 |
| 容易写错（忘记 `source`、`From`） | 编译器帮你保证正确性 |

除非有非常特殊的需求（比如自定义 `source()` 的返回逻辑），否则优先用 `thiserror`。

---

## 本题覆盖

Practice defining practical library errors with the `thiserror` crate.

## 需要重点理解

- 自定义错误类型让调用者可以 `match` 不同错误并采取不同策略。
- `#[error("...")]` 格式化字符串就是用户看到的错误消息。
- `#[from]` 让 `?` 运算符能自动转换错误类型。
- `BufReader::lines()` 的每一行都是 `Result<String, io::Error>`，读取过程中仍然可能失败。
- `thiserror` 是零成本抽象——不引入运行时开销。

## 常见坑

- 在 `#[error]` 里写复杂逻辑（如函数调用），格式化字符串应该只做简单展示。
- 忘记给变体加 `#[error(...)]`，编译器会报错但错误信息可能不太直观。
- 把 `reader.lines()` 当成 `text.lines()` 用，忘记处理每一行的 `Result`。
- 忘记 `use std::io::BufRead;`，导致 `.lines()` 方法不可用。
- 在 binary 里也定义一堆自定义错误类型——binary 用 `anyhow` 就够了。
- 混用 `thiserror` 和手动 `impl Display`，导致编译冲突。

## 回看问题

- 你的错误类型是否覆盖了所有可能的失败场景？
- 错误消息对用户来说是否足够清晰？对开发者调试是否足够详细？
- 如果有人用了你的库，他们能方便地 `match` 你返回的错误吗？

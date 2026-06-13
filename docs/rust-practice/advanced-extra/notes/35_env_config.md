# 35 Environment Config - 补充笔记

## 背景知识

### 环境变量是什么

环境变量可以理解成“启动程序时，操作系统顺手交给程序的一组字符串配置”。它适合放运行时配置，例如输入文件、日志级别、默认阈值等。

需要注意三点：

- 环境变量的值本质上不是 Rust 类型，而是字符串；要用数字时必须自己解析。
- 环境变量通常只影响当前 shell 会话和它启动的子进程。
- 不要把环境变量当成全局变量到处读；最好集中在 `parse_config` 之类的函数里读取，再把解析后的 `Config` 传给业务逻辑。

### 读取环境变量

```rust
use std::env;

let val = env::var("TEXT_INPUT"); // 返回 Result<String, env::VarError>
```

`VarError` 有两种：

- `VarError::NotPresent`：变量未设置。
- `VarError::NotUnicode`：变量存在但不是合法 UTF-8（在 Windows 上较常见）。

只关心是否存在时，可以用 `.ok()` 把错误转成 `None`：

```rust
let val: Option<String> = env::var("TEXT_INPUT").ok();
```

### 解析数字字符串

```rust
let min_count: usize = match env::var("MIN_COUNT") {
    Ok(raw) => raw
        .parse::<usize>()
        .map_err(|_| format!("invalid MIN_COUNT: {raw}"))?,
    Err(env::VarError::NotPresent) => 2,
    Err(env::VarError::NotUnicode(_)) => {
        return Err("MIN_COUNT is not valid Unicode".to_string());
    }
};
```

`.parse::<usize>()` 返回 `Result<usize, ParseIntError>`。如果 `MIN_COUNT` 没设置，可以使用默认值；如果设置了但不是合法数字，应该返回清晰错误，不要悄悄退回默认值。否则用户写错配置时会以为程序真的按自己的配置运行了。

### CLI 参数与环境变量的优先级

这题的预期逻辑：

```
CLI 第一个参数 → 如果没有，尝试 TEXT_INPUT 环境变量 → 如果也没有，返回错误
```

用 `Option::or_else` 可以很清晰地表达这个逻辑：

```rust
let input = env::args().nth(1)
    .map(PathBuf::from)
    .or_else(|| env::var("TEXT_INPUT").ok().map(PathBuf::from));
```

`env::args().nth(1)` 表示取第一个真正的用户参数，因为 `nth(0)` 是程序名本身。

如果想让结构更清楚，可以把配置解析单独写成函数：

```rust
use std::{env, path::PathBuf};

struct Config {
    input: PathBuf,
    min_count: usize,
}

fn parse_config() -> Result<Config, String> {
    let input = env::args()
        .nth(1)
        .map(PathBuf::from)
        .or_else(|| env::var("TEXT_INPUT").ok().map(PathBuf::from))
        .ok_or_else(|| "missing input: provide a CLI arg or set TEXT_INPUT".to_string())?;

    let min_count = match env::var("MIN_COUNT") {
        Ok(raw) => raw
            .parse::<usize>()
            .map_err(|_| format!("invalid MIN_COUNT: {raw}"))?,
        Err(env::VarError::NotPresent) => 2,
        Err(env::VarError::NotUnicode(_)) => {
            return Err("MIN_COUNT is not valid Unicode".to_string());
        }
    };

    Ok(Config { input, min_count })
}
```

### 如何设置环境变量运行

**Linux / macOS / Git Bash：**
```bash
TEXT_INPUT=sample.txt MIN_COUNT=3 cargo run
```

**Windows cmd：**
```cmd
set TEXT_INPUT=sample.txt && set MIN_COUNT=3 && cargo run
```

**Windows PowerShell：**
```powershell
$env:TEXT_INPUT="sample.txt"; $env:MIN_COUNT="3"; cargo run
```

### 自定义错误类型示例

这题需要在 CLI 和环境变量都缺失时返回错误。最简单的写法是返回 `Box<dyn std::error::Error>`（无需定义新类型），或者用字符串错误：

```rust
fn parse_config() -> Result<Config, String> {
    let input = // ...
        .ok_or_else(|| "missing input: provide a CLI arg or set TEXT_INPUT".to_string())?;
    // ...
}
```

更成熟的写法会到 38、39 题介绍的 `thiserror` 和 `anyhow`。

---

## 本题覆盖

Practice reading environment variables and combining them with default configuration.

## 需要重点理解

- 真实 CLI 程序不能总是假设固定文件名。
- `PathBuf` / `Path` 比 `String` 更适合表达文件系统路径。
- 大文件处理优先考虑 streaming IO。
- 配置解析应该和业务逻辑分开。

## 常见坑

- 为了图方便使用 `unwrap`，导致错误信息不清楚。
- 把 CLI、IO、业务逻辑全部写在一个函数里，后续难以测试。
- 过早引入复杂抽象，而不是先保持清晰边界。

## 回看问题

- 这个题目里的边界在哪里：CLI、IO、解析、业务逻辑还是并发/异步？
- 哪些错误应该交给调用者处理？哪些可以在 binary 层转换成用户提示？
- 当前设计是否容易写测试？

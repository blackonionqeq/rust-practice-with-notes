# 36 Clap Text Stats - 补充笔记

## 背景知识

### 为什么用 `clap`

前几题手动读取了 `std::env::args` 和环境变量，这能帮助理解底层机制。但真实 CLI 程序通常需要：

- 自动生成 `--help`。
- 把字符串参数解析成 `PathBuf`、`usize`、`bool` 等类型。
- 处理缺失参数、非法数字、未知参数，并给用户清晰提示。

`clap` 的 derive 写法让你用一个结构体描述命令行参数，解析成功后直接得到强类型字段。

### 添加依赖

本题需要启用 `derive` feature：

```bash
cargo add clap --features derive
```

或者手动在 `Cargo.toml` 中写：

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

### 最小结构

```rust
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, value_name = "PATH")]
    input: PathBuf,

    #[arg(long, default_value_t = 2)]
    min_count: usize,

    #[arg(long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
```

几个点要看懂：

- `use clap::Parser;` 把 `Args::parse()` 这个方法带进作用域。
- `#[derive(Parser)]` 让 `clap` 根据结构体生成参数解析逻辑。
- `#[arg(long)] input: PathBuf` 对应 `--input <PATH>`。
- `#[arg(long, default_value_t = 2)] min_count: usize` 对应 `--min-count <N>`，没传时是 `2`。
- `ignore_case: bool` 配合 `#[arg(long)]` 是开关参数：传 `--ignore-case` 就是 `true`，不传就是 `false`。

字段名里的下划线会自动转换成命令行里的短横线，所以 `min_count` 对应 `--min-count`，`ignore_case` 对应 `--ignore-case`。

### 运行示例

```bash
cargo run -- --input sample.txt
cargo run -- --input sample.txt --min-count 3
cargo run -- --input sample.txt --min-count 3 --ignore-case
cargo run -- --help
```

`cargo run --` 里的第一个 `--` 是 Cargo 和你的程序之间的分隔符。它后面的内容才会传给你的程序。

### 和手动解析的区别

前几题里你可能会写：

```rust
let input = std::env::args().nth(1);
```

这题不要这么做。目标是练习让 `clap` 负责解析命令行，再把解析后的 `Args` 交给你的业务逻辑：

```rust
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // 读取 args.input，按 args.min_count 和 args.ignore_case 做统计
    Ok(())
}
```

这样 CLI 边界和文本统计逻辑是分开的，后面写测试也更容易。

### 错误处理边界

`clap` 会自动处理命令行格式错误，例如：

- 缺少 `--input`。
- `--min-count abc` 不是数字。
- 传了不存在的参数。

这些错误通常不需要你自己 `unwrap` 或手写提示；`Args::parse()` 会打印错误并退出。你需要自己处理的是应用逻辑里的错误，例如文件打不开、读文件失败、统计逻辑异常等。

## 本题覆盖

Practice using the `clap` crate for a small typed CLI.

## 需要重点理解

- 真实 CLI 程序不能总是假设固定文件名。
- `PathBuf` / `Path` 比 `String` 更适合表达文件系统路径。
- 大文件处理优先考虑 streaming IO。
- 配置解析应该和业务逻辑分开。
- `clap` 负责命令行解析，不等于负责你的业务逻辑。
- `--min-count` 这种命令行名字来自 Rust 字段名 `min_count`。

## 常见坑

- 为了图方便使用 `unwrap`，导致错误信息不清楚。
- 把 CLI、IO、业务逻辑全部写在一个函数里，后续难以测试。
- 过早引入复杂抽象，而不是先保持清晰边界。
- 忘记启用 `derive` feature，导致 `#[derive(Parser)]` 或 `Args::parse()` 不可用。
- 忘记 `use clap::Parser;`，导致编译器找不到 `parse` 方法。
- 在 `cargo run` 后少写分隔符 `--`，参数被 Cargo 吃掉或解释错。
- 继续手动解析 `std::env::args`，没有练到本题核心。

## 回看问题

- 这个题目里的边界在哪里：CLI、IO、解析、业务逻辑还是并发/异步？
- 哪些错误应该交给调用者处理？哪些可以在 binary 层转换成用户提示？
- 当前设计是否容易写测试？
- 哪些错误已经由 `clap` 处理？哪些仍然需要你的 `run` 函数返回？

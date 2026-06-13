# 54 Tokio File Read - 补充笔记

## 背景知识

### 为什么有 tokio::fs

`std::fs::read_to_string` 是**阻塞**调用——读文件时整个线程都卡住等着。在 async 程序里用它会占用 runtime 的 worker 线程，拖慢并发。

`tokio::fs` 提供异步版本的文件 API，签名和 `std::fs` 很像，但它是 async 的：

```rust
use tokio::fs;

async fn read_text(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path).await
}
```

> 小知识：`tokio::fs` 底层其实还是把阻塞 IO 丢到一个线程池里跑，对使用者屏蔽了细节。重点是**API 层面它是 async 的**，可以和 `.await`、`join!` 等无缝配合。

### async 函数的错误传播

和同步函数一样，async 函数里也能用 `?` 把错误向上抛。返回类型用 `Result<T, E>`：

```rust
async fn read_text(path: &Path) -> std::io::Result<String> {
    let content = tokio::fs::read_to_string(path).await?;  // ? 传播 io::Error
    Ok(content)
}
```

调用方用 `.await` 拿到 `Result`，再决定怎么处理，不要 `unwrap`。

### 划清 IO 边界：异步只用在真正的 IO 上

把"读文件"这种 IO 交给 async，但**纯计算**（数行数、数单词）保持同步即可——它们本来就不阻塞，没必要包成 async：

```rust
// IO：异步
let text = read_text(&path).await?;

// 纯计算：同步就好，没必要 async
let lines = text.lines().count();
let words = text.split_whitespace().count();
println!("{} 行, {} 词", lines, words);
```

乱把所有函数都标成 async 只会增加无谓的 `Future` 包装和状态机开销。

### 从 CLI 读路径

和同步版本一样，用 `env::args` 拿命令行参数，用 `PathBuf` 表达路径：

```rust
use std::{env, path::PathBuf};

let path = env::args()
    .nth(1)
    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "缺少文件路径"))?;
let path = PathBuf::from(path);
```

传给接受 `&Path` 的函数时直接传 `&path`。

### Cargo.toml 里启用 fs feature

`tokio::fs` 需要在依赖里启用 `fs` feature：

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "fs"] }
```

`macros` 提供 `#[tokio::main]`，`rt-multi-thread` 提供多线程 runtime，`fs` 提供异步文件 API。

---

## 本题覆盖

Practice async file reading with `tokio::fs`.

## 需要重点理解

- `tokio::fs` 是 `std::fs` 的异步对应物，避免阻塞 runtime 线程。
- async 函数同样能用 `?` 传播错误，返回 `Result<T, E>`。
- 异步只用在真正的 IO 上，纯计算保持同步。
- `tokio::fs` 需要在 `Cargo.toml` 里启用 `fs` feature。

## 常见坑

- 在 async 代码里直接用 `std::fs::read_to_string`，阻塞了整个 runtime。
- 把数行数、数单词这类纯计算也写成 async，徒增开销。
- 对 `.await` 出来的 `Result` 用 `unwrap`，文件不存在时直接 panic。
- 忘了在 `Cargo.toml` 启用 `fs` feature，编译报找不到 `tokio::fs`。

## 回看问题

- 这道题里哪些是异步 IO、哪些是纯计算？边界划得清楚吗？
- 用 `std::fs` 替换 `tokio::fs` 会带来什么问题？
- 文件不存在或无权限时，你的 async 函数返回的错误足够清晰吗？

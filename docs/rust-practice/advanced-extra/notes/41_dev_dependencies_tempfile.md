# 41 Dev Dependencies and Tempfile - 补充笔记

## 背景知识

### Cargo.toml 里的三种依赖

```toml
[dependencies]
serde = "1"          # 正式依赖：编译和运行都需要

[dev-dependencies]
tempfile = "3"       # 开发依赖：只在测试和 bench 时编译

[build-dependencies]
cc = "1"             # 构建依赖：编译 build.rs 时需要
```

关键区别：

| 类型 | 什么时候编译 | 谁能用 | 发布时包含 |
|------|------------|--------|-----------|
| `dependencies` | 总是 | 所有代码 | ✅ 是 |
| `dev-dependencies` | `cargo test` / `cargo bench` | 只有 `tests/`、`benches/` 和 `#[cfg(test)]` 模块 | ❌ 否 |
| `build-dependencies` | 编译 `build.rs` 时 | 只有 `build.rs` | ❌ 否 |

如果你的 crate 被别人引用，你的 `dev-dependencies` **不会**传递给对方——它们只在你本地开发和 CI 时存在。

### 为什么需要 dev-dependencies

测试经常需要一些"辅助工具"——临时文件、假 HTTP 服务器、随机数据生成器——这些不应该出现在正式依赖里：

- **减小编译体积**：用户 `cargo build` 时不用编译测试工具。
- **减少供应链风险**：少一个依赖就少一个安全漏洞的可能性。
- **语义清晰**：看 `Cargo.toml` 就能分清"运行必需"和"只测一下"。

### tempfile crate

测试文件操作时，你不想污染文件系统。`tempfile` 提供两种临时文件抽象：

#### tempfile::NamedTempFile —— 有名字的临时文件

```rust
use tempfile::NamedTempFile;
use std::io::Write;

let mut tmp = NamedTempFile::new()?;       // 在系统临时目录创建
writeln!(tmp, "hello")?;                   // 写入内容
let path = tmp.path().to_path_buf();       // 拿到路径，传给被测代码
// tmp 被 drop 时自动删除文件
```

也可以指定后缀 / 前缀，方便调试：

```rust
let tmp = NamedTempFile::with_suffix(".json")?;
// 文件名类似 /tmp/xxxxx.json
```

#### tempfile::TempDir —— 临时目录

```rust
use tempfile::TempDir;

let dir = TempDir::new()?;                 // 创建临时目录
let file_path = dir.path().join("config.json");
std::fs::write(&file_path, r#"{"port":8080}"#)?;

let config = load_config(&file_path)?;     // 用被测函数读取
// dir 被 drop 时整个目录及其内容自动删除
```

### 自动清理的原理

`NamedTempFile` 和 `TempDir` 都利用了 Rust 的 `Drop` trait。当变量离开作用域时，析构函数自动删除文件/目录：

```rust
fn test_something() -> anyhow::Result<()> {
    let tmp = NamedTempFile::new()?;
    // ... 使用 tmp
    Ok(())
}   // ← 这里 tmp 被 drop，文件被删除
```

即使测试 panic 了，`Drop` 也会执行（除非进程被 `SIGKILL` 等强制终止），所以不会留下垃圾文件。

### #[cfg(test)] 模块

测试辅助代码应该放在 `#[cfg(test)]` 模块里，这样正式编译时完全不存在：

```rust
// src/lib.rs
pub fn parse_config(text: &str) -> Result<Config, AppError> {
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_parse_valid_config() -> anyhow::Result<()> {
        let mut tmp = NamedTempFile::new()?;
        writeln!(tmp, r#"{{"host":"localhost","port":8080}}"#)?;
        let text = std::fs::read_to_string(tmp.path())?;
        let config = parse_config(&text)?;
        assert_eq!(config.host, "localhost");
        Ok(())
    }
}
```

`#[cfg(test)]` 保证了 `tempfile` 只在测试时被引用——恰好配合 `dev-dependencies`。

### 手动写临时文件的痛点

不用 `tempfile` 时，你可能这样写：

```rust
// 手动管理临时文件——容易出问题
let path = "/tmp/test_config.json";
std::fs::write(path, content)?;
// 测试 ...
std::fs::remove_file(path)?;  // 如果测试 panic 了，这行不会执行
```

问题：
- 并行测试时文件名冲突。
- 测试 panic 后文件留在磁盘上。
- 不同 OS 的临时目录不同（Windows 是 `%TEMP%`，Linux 是 `/tmp`）。
- 忘记清理。

`tempfile` 一次性解决所有这些问题。

### 测试里的错误处理

测试函数可以返回 `Result`：

```rust
#[test]
fn test_something() -> anyhow::Result<()> {
    let tmp = NamedTempFile::new()?;
    // ... 如果任何 ? 出错，测试自动标记为 failed
    Ok(())
}
```

比在测试里用 `.unwrap()` 好得多——错误消息会包含具体的失败原因，而不是只看到 "called unwrap on None"。

---

## 本题覆盖

Practice using `dev-dependencies` and temporary files in tests.

## 需要重点理解

- `dev-dependencies` 只在测试时编译，不会传递给依赖你的用户。
- `tempfile` 利用 `Drop` 自动清理，即使测试 panic 也不会留下垃圾文件。
- `#[cfg(test)]` 让测试辅助代码完全不影响正式编译。
- 测试函数可以返回 `Result`，用 `?` 代替 `.unwrap()`。

## 常见坑

- 在正式代码（非 `#[cfg(test)]`）里引用了 `dev-dependencies` 的类型——编译不过。
- 用硬编码路径（如 `/tmp/test`）而不是 `tempfile`——并行测试时冲突。
- 测试里到处 `.unwrap()`，出错时看不到具体原因。
- 在 `TempDir` 被 drop 之后还尝试访问里面的文件。

## 回看问题

- 你的测试是否真的独立？运行两次会不会因为残留文件而失败？
- `dev-dependencies` 里的 crate 是否只用在测试里？有没有误用到正式代码？
- 如果并行跑所有测试，会不会因为共享资源（文件、端口、数据库）而冲突？

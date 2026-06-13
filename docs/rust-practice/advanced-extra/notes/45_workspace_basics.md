# 45 Workspace Basics - 补充笔记

## 背景知识

### 为什么需要 Workspace

一个项目做大后，你可能会发现：
- 有些代码是"库"（解析器、工具函数）。
- 有些代码是"应用"（CLI 工具、HTTP 服务）。
- 它们共享一些依赖，但分开编译又各自独立。

如果全部塞在一个 crate 里，编译会越来越慢，职责也不清晰。**Workspace** 让你把多个 crate 放在同一个仓库里，共享一个 `Cargo.lock`，但各自有独立的 `Cargo.toml`。

### Workspace 的目录结构

```
my-project/
├── Cargo.toml          ← workspace 的根配置（虚拟清单）
├── Cargo.lock          ← 共享的锁文件（只生成一份）
├── crates/
│   ├── my-lib/
│   │   ├── Cargo.toml  ← 成员 crate 的配置
│   │   └── src/
│   │       └── lib.rs
│   └── my-cli/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
└── target/             ← 共享的编译输出目录
```

### 根 Cargo.toml（虚拟清单）

```toml
# my-project/Cargo.toml
[workspace]
members = [
    "crates/my-lib",
    "crates/my-cli",
]
resolver = "2"
```

注意：这个 `Cargo.toml` **没有** `[package]` 部分。它只定义了 workspace 的成员列表。这种没有 `[package]` 只有 `[workspace]` 的 `Cargo.toml` 叫做**虚拟清单（Virtual Manifest）**。

### 成员 crate 的 Cargo.toml

```toml
# crates/my-lib/Cargo.toml
[package]
name = "my-lib"
version = "0.1.0"
edition = "2021"
```

```toml
# crates/my-cli/Cargo.toml
[package]
name = "my-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
my-lib = { path = "../my-lib" }   # 引用同 workspace 的另一个 crate
anyhow = "1"
```

成员之间用 `path = "..."` 互相引用，不需要发布到 crates.io。

### 共享依赖版本

如果多个成员都用 `serde`，你可以在 workspace 根部统一定义版本：

```toml
# 根 Cargo.toml
[workspace]
members = ["crates/*"]
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
anyhow = "1"
```

```toml
# crates/my-lib/Cargo.toml
[dependencies]
serde = { workspace = true }   # 继承 workspace 的版本和 features
```

```toml
# crates/my-cli/Cargo.toml
[dependencies]
serde = { workspace = true }
my-lib = { path = "../my-lib" }
```

好处：所有成员用同一个版本的 `serde`，不会出现版本冲突，也只需要编译一次。

### 常用 Workspace 命令

```bash
# 编译整个 workspace
cargo build

# 只编译某一个成员
cargo build -p my-lib

# 运行某个 binary crate
cargo run -p my-cli

# 测试整个 workspace
cargo test

# 只测试某一个成员
cargo test -p my-lib

# 查看所有成员
cargo metadata --format-version 1 | jq '.workspace_members'
```

### 共享 target/ 目录

Workspace 里所有成员的编译产物都放在根目录的 `target/` 下，而不是每个 crate 各自一份。这意味着：
- `my-lib` 编译过的依赖，`my-cli` 不用再编译。
- 删 `target/` 一次性清理所有成员。

### workspace 里成员之间的依赖

```toml
# crates/my-cli/Cargo.toml
[dependencies]
my-lib = { path = "../my-lib" }
```

这告诉 Cargo：`my-cli` 依赖同 workspace 里的 `my-lib`。Cargo 知道它们的依赖关系，会按正确顺序编译。如果 `my-lib` 的代码改了，再编译 `my-cli` 时会自动重新编译 `my-lib`。

### 什么时候该拆 Workspace

| 情况 | 建议 |
|------|------|
| 单个小项目 | 不需要，一个 crate 就够 |
| 库 + CLI 工具 | 拆成两个 crate：一个 lib，一个 bin |
| 多个互相独立的服务 | 每个服务一个 crate |
| 共享代码 + 多个前端 | 核心 lib + 各前端的 bin |

原则：**当编译时间变长、职责开始混杂时，就该拆了**。过早拆分反而增加管理成本。

---

## 本题覆盖

Practice organizing multiple crates in one Cargo workspace.

## 需要重点理解

- Workspace 让多个 crate 共享一个 `Cargo.lock` 和 `target/`，节省编译时间和磁盘空间。
- `[workspace.dependencies]` 统一管理依赖版本，避免成员间版本冲突。
- 成员之间用 `path = "..."` 互相引用，不需要发布到 crates.io。
- 虚拟清单（只有 `[workspace]` 没有 `[package]`）是 workspace 的根配置。

## 常见坑

- 根 `Cargo.toml` 误加了 `[package]`，导致 Cargo 报 "cannot have both [workspace] and [package]"。
- `path = "..."` 写错了相对路径——相对于当前 `Cargo.toml` 所在目录。
- 在 `Cargo.lock` 上各搞各的——workspace 里应该只有一个根 `Cargo.lock`。
- 拆得太细（一个函数一个 crate），维护成本远大于收益。

## 回看问题

- 你的 workspace 拆分是否反映了合理的职责边界？
- 成员之间的依赖方向是否合理？（应该避免循环依赖）
- 新成员加入项目时，目录结构是否容易理解？

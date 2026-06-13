# 46 Feature Flags - 补充笔记

## 背景知识

### 什么是 Feature Flag

Feature flag（特性标志）让你的 crate 提供可选功能。用户可以按需启用，不用时不会编译进去：

```toml
# 用户的 Cargo.toml
[dependencies]
my-crate = { version = "1.0", features = ["json"] }  # 启用 JSON 支持
```

```toml
# 你的 crate 的 Cargo.toml
[features]
default = []          # 默认不启用任何 feature
json = ["serde"]      # 启用 "json" 时自动拉入 serde 依赖

[dependencies]
serde = { version = "1", optional = true }  # 可选依赖
```

当用户不启用 `json` feature 时，`serde` 完全不会被编译。

### 定义 Feature

```toml
[features]
# 简单 feature：不依赖其他 feature 或 crate
logging = []

# 依赖其他 feature
full = ["json", "yaml", "logging"]

# 依赖可选 crate（名字和 [dependencies] 里的 key 一致）
json = ["serde", "serde_json"]
yaml = ["serde_yaml"]
```

### 可选依赖（Optional Dependencies）

在 `[dependencies]` 里加 `optional = true`，这个依赖就变成了可选的——只有对应的 feature 被启用时才会编译：

```toml
[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
serde_json = { version = "1", optional = true }
rayon = { version = "1.8", optional = true }   # 并行计算支持
```

每个 `optional = true` 的依赖会自动生成一个同名的 feature。启用 `serde` feature → `serde` crate 被编译。

### 在代码里用条件编译

```rust
#[cfg(feature = "json")]
pub fn parse_json(text: &str) -> Result<Config, serde_json::Error> {
    serde_json::from_str(text)
}

#[cfg(feature = "yaml")]
pub fn parse_yaml(text: &str) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_str(text)
}

// 不论启用哪个 feature 都能用
pub fn default_port() -> u16 {
    8080
}
```

`#[cfg(feature = "json")]` 意味着：只有启用 `json` feature 时这个函数才存在。没有启用就调用会编译报错。

### cfg 宏与 cfg_attr

```rust
// cfg：条件编译整个 item
#[cfg(feature = "json")]
fn parse_json(text: &str) -> Config { ... }

// cfg_attr：条件性地应用属性
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Config {
    host: String,
    port: u16,
}
```

`cfg_attr` 的意思是：如果启用了 `serde` feature，就加上 `derive(Serialize, Deserialize)`；否则这个 struct 就没有这两个 derive。

### 模块级别的条件编译

```rust
// src/lib.rs
#[cfg(feature = "json")]
mod json_support;

#[cfg(feature = "yaml")]
mod yaml_support;

pub mod config;  // 始终可用
```

整个模块可以在不需要时完全不编译。

### default features

```toml
[features]
default = ["json"]   # cargo build 默认启用 json
json = ["serde"]
yaml = ["serde_yaml"]
```

```bash
# 默认构建（启用 json）
cargo build

# 不启用任何默认 feature
cargo build --no-default-features

# 只启用指定 feature
cargo build --no-default-features --features yaml

# 同时启用多个 feature
cargo build --features "json,yaml"
```

最佳实践：**`default` 应该尽量少**。默认构建应该是轻量的，让用户按需加功能。

### 测试不同的 feature 组合

```bash
# 测试默认 features
cargo test

# 测试不带默认 features
cargo test --no-default-features

# 测试特定 feature
cargo test --features json

# 测试所有 feature 的组合（CI 里推荐）
cargo test --all-features
```

在 CI 里应该至少跑一次 `cargo test --all-features`，确保所有 feature 组合都能编译。

### feature 的命名惯例

- 小写，用 `-` 分隔（`json-support`，不是 `json_support`）。
- 名字应该描述"提供了什么能力"，而不是"用了什么技术"。
  - ✅ 好：`parallel`、`json`、`tls`
  - ❌ 差：`uses_rayon`、`with_serde`

### 常见的 feature 用法

```toml
[features]
default = []

# 格式支持
json = ["serde", "serde_json"]
yaml = ["serde_yaml"]
toml = ["toml-crate"]

# 功能开关
parallel = ["rayon"]           # 并行计算
tls = ["native-tls"]           # TLS 加密

# 便捷组合
full = ["json", "yaml", "toml", "parallel", "tls"]
```

用户可以按需选择：
- 只要 JSON 支持：`features = ["json"]`
- 要全部功能：`features = ["full"]`
- 最小化：`default-features = false, features = []`

---

## 本题覆盖

Practice optional functionality controlled by Cargo features.

## 需要重点理解

- Feature flag 通过 `#[cfg(feature = "...")]` 实现编译期条件包含。
- `optional = true` 的依赖只有在对应 feature 启用时才编译。
- `default` features 决定了 `cargo build` 默认启用什么——应该尽量少。
- `--all-features` 在 CI 里确保所有组合都能编译。

## 常见坑

- feature 名和依赖名冲突（比如依赖叫 `serde`，feature 也叫 `serde`，虽然可以用但容易混淆）。
- 忘记在 CI 里测 `--no-default-features`，导致某些 feature 组合编译不过。
- feature 之间有隐式依赖关系但没有在 `[features]` 里声明，用户启用 A 忘了 B 导致编译失败。
- `default` 里塞了太多东西，用户想最小化安装时很麻烦。

## 回看问题

- 你的 `default` features 是否足够轻量？用户能轻松做最小化安装吗？
- feature 之间的依赖关系是否在 `[features]` 里显式声明了？
- 如果两个 feature 被同时启用，会不会冲突？

# 40 Serde JSON Config - 补充笔记

## 背景知识

### 什么是序列化 / 反序列化

- **序列化（Serialize）**：把 Rust 数据结构转成某种格式（JSON、YAML、TOML……）。
- **反序列化（Deserialize）**：把格式化文本转回 Rust 数据结构。

```rust
// 序列化：Rust → JSON
let config = Config { port: 8080, host: "localhost".into() };
let json = serde_json::to_string(&config)?;
// -> "{\"port\":8080,\"host\":\"localhost\"}"

// 反序列化：JSON → Rust
let config: Config = serde_json::from_str(r#"{"port":8080,"host":"localhost"}"#)?;
```

为什么不用字符串解析？因为你不需要手写解析器——`serde` 帮你处理了格式语法、转义字符、类型转换，你只需要定义结构体。

### serde 和 serde_json 的关系

- **`serde`**：核心框架，定义了 `Serialize` 和 `Deserialize` 两个 trait。
- **`serde_json`**：具体格式实现，负责 JSON 的读写。

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }   # 提供 derive 宏
serde_json = "1"                                     # JSON 格式支持
```

`features = ["derive"]` 是必须的——没有它就没有 `#[derive(Serialize, Deserialize)]`。

### 基本用法

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

fn main() -> anyhow::Result<()> {
    // 从 JSON 文件读取配置
    let text = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&text)?;
    println!("Connecting to {}:{} (debug={})", config.host, config.port, config.debug);
    Ok(())
}
```

对应的 `config.json`：

```json
{
    "host": "localhost",
    "port": 8080,
    "debug": true
}
```

字段名必须完全匹配（大小写敏感）。

### 字段重命名和默认值

JSON 里用 `snake_case`，但你的 API 规范要求 `camelCase`？用 `#[serde(rename)]`：

```rust
#[derive(Deserialize)]
struct Config {
    #[serde(rename = "maxRetries")]
    max_retries: u32,
}
```

某些字段可选？加 `#[serde(default)]`，缺失时用类型的默认值：

```rust
#[derive(Deserialize)]
struct Config {
    host: String,
    #[serde(default = "default_port")]
    port: u16,
}

fn default_port() -> u16 { 8080 }
```

或者直接用 `Option`：

```rust
#[derive(Deserialize)]
struct Config {
    host: String,
    port: Option<u16>,  // 缺失时为 None
}
```

### 嵌套结构

```rust
#[derive(Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}
```

对应 JSON：

```json
{
    "server": {
        "host": "localhost",
        "port": 8080
    },
    "database": {
        "url": "postgres://localhost/mydb",
        "max_connections": 10
    }
}
```

结构体和 JSON 的嵌套层级一一对应。

### 反序列化时的常见错误

```rust
// 字段名不匹配 → Error: missing field `port`
// 类型不匹配 → Error: invalid type: string "abc", expected u16
// JSON 语法错误 → Error: expected `,` at line 3 column 5
```

结合 `anyhow::context()` 可以给用户更友好的提示：

```rust
let config: Config = serde_json::from_str(&text)
    .context("config.json 格式错误，请检查语法")?;
```

### serde_json 的读写函数

| 函数 | 方向 | 输入/输出 |
|------|------|----------|
| `serde_json::from_str(s)` | JSON → Rust | `&str` → `Result<T>` |
| `serde_json::from_slice(b)` | JSON → Rust | `&[u8]` → `Result<T>` |
| `serde_json::from_reader(r)` | JSON → Rust | `impl Read` → `Result<T>` |
| `serde_json::to_string(&v)` | Rust → JSON | → `Result<String>` |
| `serde_json::to_string_pretty(&v)` | Rust → JSON（带缩进） | → `Result<String>` |
| `serde_json::to_writer(w, &v)` | Rust → JSON | 写到 `impl Write` |

`from_reader` 可以直接从文件反序列化，跳过中间的 `String`：

```rust
let file = std::fs::File::open("config.json")?;
let config: Config = serde_json::from_reader(file)?;
```

### serde::Value —— 无模式 JSON

当你不知道 JSON 的结构时，可以用 `serde_json::Value`：

```rust
let val: serde_json::Value = serde_json::from_str(&text)?;
if let Some(port) = val["port"].as_u64() {
    println!("port = {port}");
}
```

但更推荐定义结构体——类型系统在编译时帮你检查字段名和类型，比运行时才知道错了要好得多。

---

## 本题覆盖

Practice parsing a JSON configuration file into typed Rust structs.

## 需要重点理解

- `#[derive(Deserialize)]` 是零成本抽象，编译期生成所有解析代码。
- 结构体字段名必须和 JSON key 完全匹配，或用 `#[serde(rename)]` 映射。
- 可选字段用 `Option<T>` 或 `#[serde(default)]`。
- 配置文件解析失败应该给用户清晰的错误提示，而不是 panic。

## 常见坑

- 忘记在 `Cargo.toml` 里给 serde 加 `features = ["derive"]`，导致 `#[derive(Serialize)]` 不认识。
- JSON key 和结构体字段名大小写不匹配，反序列化报 `missing field`。
- 用 `serde_json::Value` 到处索引（`val["a"]["b"]`），丢失了类型安全。
- 大文件用 `from_str` 一次性加载到内存——超大文件应该用 streaming API。

## 回看问题

- 你的配置结构体是否覆盖了所有需要的字段？缺省值合理吗？
- 如果配置文件格式变了（比如从 JSON 迁移到 TOML），代码改动有多大？
- 反序列化错误是否给了用户足够的信息去修正配置？

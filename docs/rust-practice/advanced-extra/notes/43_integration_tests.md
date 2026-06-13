# 43 Integration Tests - 补充笔记

## 背景知识

### 单元测试 vs 集成测试

Rust 有两种测试方式：

| | 单元测试 | 集成测试 |
|--|---------|---------|
| 位置 | `src/` 里的 `#[cfg(test)] mod tests` | 项目根目录的 `tests/` 文件夹 |
| 可见性 | 能访问 `pub(crate)` 和私有函数 | 只能访问 `pub` API |
| 运行方式 | `cargo test`（一起跑） | `cargo test`（一起跑） |
| 本质 | 和源码在同一个 crate 里编译 | 把你的 crate 当作外部依赖来用 |

```rust
// ===== 单元测试：src/lib.rs =====
pub fn add(a: i32, b: i32) -> i32 { a + b }

fn internal_helper(x: i32) -> i32 { x * 2 }  // 私有函数

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal() {
        assert_eq!(internal_helper(3), 6);  // ✅ 能测私有函数
    }
}
```

```rust
// ===== 集成测试：tests/integration.rs =====
use my_crate::add;  // 像"用户"一样引用

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);  // ✅ 只能用公开 API
}
```

### tests/ 目录的结构

```
my_crate/
├── src/
│   └── lib.rs
└── tests/
    ├── common.rs       ← 辅助模块（不会被当作独立测试文件）
    ├── common/
    │   └── mod.rs      ← 同上，另一种写法
    ├── api_test.rs     ← 集成测试文件
    └── error_test.rs   ← 集成测试文件
```

Cargo 的规则：
- `tests/` 下的每个 `.rs` 文件 → 编译成独立的测试 binary。
- `tests/common/` 目录或 `tests/common.rs` 中的 `mod` → 不会自动变成测试，可以被其他文件 `mod common;` 引用。

### 为什么需要集成测试

单元测试验证内部逻辑，集成测试验证**用户视角**：

```rust
// 单元测试关注内部细节
#[test]
fn parser_handles_empty_input() {
    assert!(parse("").is_err());
}

// 集成测试关注"用了这个库能不能正确工作"
#[test]
fn user_can_load_config_from_file() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    // 写入配置 → 调用公开 API → 验证结果
    let config = my_crate::load_config(tmp.path()).unwrap();
    assert_eq!(config.port, 8080);
}
```

如果你的 `pub` API 设计得不好，集成测试会**先**暴露出来——因为你只能用公开接口，如果"不好用"，测试写起来就会很别扭。这就是为什么 TDD（测试驱动开发）推荐先写集成测试。

### 集成测试里引用被测 crate

```rust
// tests/api_test.rs
use my_crate::{Config, load_config};  // 和用户一样用 use

// 也可以整体引入
use my_crate::*;
```

注意：集成测试里 `use` 的名字必须是被测 crate 的 `pub` 导出项。如果你发现某个东西"测不到"，说明它不是 `pub` 的——这正是集成测试的意义。

### 共享测试辅助代码

多个集成测试文件需要相同的 setup 代码时，放到 `tests/common/mod.rs`：

```rust
// tests/common/mod.rs
pub fn create_test_config() -> tempfile::NamedTempFile {
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    use std::io::Write;
    writeln!(tmp, r#"{{"host":"localhost","port":8080}}"#).unwrap();
    tmp
}
```

```rust
// tests/api_test.rs
mod common;  // 引入共享模块

#[test]
fn test_load() {
    let tmp = common::create_test_config();
    let config = my_crate::load_config(tmp.path()).unwrap();
    assert_eq!(config.port, 8080);
}
```

### 只运行某一个集成测试文件

```bash
# 运行 tests/api_test.rs 里的所有测试
cargo test --test api_test

# 运行某个具体的测试函数
cargo test --test api_test test_load
```

`--test` 后面跟文件名（不含 `.rs`），不是完整路径。

### 集成测试中的错误处理

和单元测试一样，集成测试函数也可以返回 `Result`：

```rust
#[test]
fn test_full_workflow() -> anyhow::Result<()> {
    let tmp = common::create_test_config();
    let config = my_crate::load_config(tmp.path())?;  // 用 ? 代替 unwrap
    assert_eq!(config.host, "localhost");
    Ok(())
}
```

需要在 `dev-dependencies` 里加 `anyhow`（如果要用的话）。

---

## 本题覆盖

Practice testing a public library API from the `tests/` directory.

## 需要重点理解

- 集成测试把你自己的 crate 当作外部依赖——只能用 `pub` API。
- 如果集成测试很难写，说明公开 API 可能设计得不好。
- `tests/common/` 用来放共享的测试辅助代码，不会自动变成测试。
- `cargo test --test <name>` 只跑某一个集成测试文件。

## 常见坑

- 在集成测试里试图 `use` 私有函数——编译不过。
- 把所有测试都塞在一个集成测试文件里——应该按功能模块拆分。
- 忘记 `mod common;` 导致辅助函数找不到。
- 集成测试和单元测试测同一件事——单元测试覆盖内部细节，集成测试覆盖用户场景。

## 回看问题

- 你的 `pub` API 是否足够让用户完成常见任务？是否暴露了不必要的内部细节？
- 集成测试是否覆盖了最重要的使用场景？
- 如果去掉单元测试只保留集成测试，你还能对正确性有多少信心？

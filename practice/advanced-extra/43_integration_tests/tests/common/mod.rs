//! 多个集成测试文件共享的辅助代码。
//!
//! 关键点：放在 `tests/common/` 目录（`mod.rs`）里，Cargo 不会把它
//! 编译成独立的测试 binary；其它测试文件通过 `mod common;` 引入。
//! 这里只放「辅助函数 / 测试夹具」，绝不放 `#[test]`。

/// 返回一段固定的示例文本，供多个测试复用。
pub fn sample_text() -> &'static str {
    "Hello, World!"
}

/// 一段纯标点文本，用来验证 `normalize` 会把首尾的非字母数字全部裁掉。
pub fn punctuation_only() -> &'static str {
    "!!!???..."
}

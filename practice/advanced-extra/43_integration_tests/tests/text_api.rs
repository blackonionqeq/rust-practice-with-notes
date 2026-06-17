//! 集成测试：像外部用户一样使用本 crate 的公开 API。
//!
//! 这里只调用 `pub` 函数，不接触任何内部实现细节。

mod common; // 引入共享辅助模块 tests/common/mod.rs

use integration_tests::{count_words, normalize};

// ---- normalize ----

// 设计有意义的，测试各个角度的用例
#[test]
fn normalize_lowercases_text() {
    assert_eq!(normalize("Hello, World!"), "hello, world");
}

#[test]
fn normalize_strips_surrounding_punctuation_but_keeps_inner() {
    // 头尾的非字母数字被裁掉，中间的逗号保留
    assert_eq!(normalize("!!!hi,there!!!"), "hi,there");
}

#[test]
fn normalize_collapses_pure_punctuation_to_empty() {
    assert_eq!(normalize(""), "");
    assert_eq!(normalize(common::punctuation_only()), "");
}

// ---- count_words ----

#[test]
fn count_words_counts_on_whitespace() {
    assert_eq!(count_words("hello world"), 2);
}

#[test]
fn count_words_ignores_extra_whitespace() {
    assert_eq!(count_words("  hello   world  "), 2);
}

#[test]
fn count_words_is_zero_for_empty_input() {
    assert_eq!(count_words(""), 0);
}

// ---- 端到端：把多个公开 API 串起来，模拟真实用户场景 ----

#[test]
fn normalize_then_count_matches_user_flow() {
    let normalized = normalize(common::sample_text());
    assert_eq!(count_words(&normalized), 2);
}

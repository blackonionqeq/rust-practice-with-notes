// 用 //! 注释crate本身，可以在引用crate时看到说明
//! 一个演示用的小型文本处理库。
//!
//! 只暴露公开 API；集成测试在 `tests/` 目录下，把本 crate 当作外部依赖来用。

// 用/// 注释可以让引用时能看到说明
/// 文本归一化：转成小写，并去掉**首尾**的非字母数字字符。
/// 中间的标点（例如逗号、空格）会原样保留。
pub fn normalize(text: &str) -> String {
    text.to_lowercase()
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_string()
}

/// 统计单词数：按任意空白字符切分后计数。
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

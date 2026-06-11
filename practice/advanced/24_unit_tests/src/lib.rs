// cargo test 会编译并运行 cfg(test) 标记的测试模块；cargo build 默认不会编译这些代码。
#[cfg(test)]
mod tests {
    use super::*;

    // #[test] 标记具体的测试函数，让 Rust 测试框架在 cargo test 时自动调用它。
    #[test]
    fn normalizes_uppercase() {
        assert_eq!(normalize_word("HELLO, WORLD!"), "hello, world");
    }

    #[test]
    fn normalizes_punctuation() {
        assert_eq!(normalize_word(",,,"), "");
    }

    #[test]
    fn normalizes_multi_lines() {
        assert_eq!(count_words("123\n456"), 2);
    }

    #[test]
    fn normalizes_empty() {
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   \n\t  "), 0);
    }
}

pub fn normalize_word(word: &str) -> String {
    // trim_matches 可以接收多种 pattern；但传闭包时，闭包签名是 FnMut(char) -> bool，
    // 所以参数只能是 char，需要显式标注类型来帮助编译器推断。
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

pub fn count_words(text: &str) -> usize {
    text.split_whitespace()
        .map(normalize_word)
        .filter(|word| !word.is_empty())
        .count()
}

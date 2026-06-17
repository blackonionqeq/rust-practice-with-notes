//! 核心区别：
//!
//!  `#`：隐藏显示，但仍编译
//!  `ignore`：不编译也不运行
//!  `//`：Rust 注释，编译器能接受

/// 文本归一化：转成小写，并去掉**首尾**的非字母数字字符。
/// 中间的标点（例如逗号、空格）会原样保留。
// 注意doc代码前后要加 ```
// 注意还要use 才行，use的得是pub才行
// 用cargo test --doc 来测试doc代码
// cargo test 会运行单元测试 + doc测试
/// ```rust
/// use doc_tests::normalize;
/// assert_eq!(normalize("Hello, World!"), "hello, world");
/// ```
pub fn normalize(input: &str) -> String {
    input
        .to_lowercase()
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_string()
}

/// 统计单词数：按任意空白字符切分后计数。
/// ```rust
/// use doc_tests::count_words;
/// assert_eq!(count_words("Hello, World!"), 2);
/// ```
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

use std::fs::File;
/// 打开文件：
/// ```rust
/// use doc_tests::open_file;
///
/// assert!(open_file("src/lib.rs").is_ok());
/// ```
/// `#` 开头的行只是在文档里隐藏，doc test 编译时仍然会执行；所以去掉 `#` 后必须是合法 Rust 代码。
///
/// ```rust
/// # // 这一行不会出现在文档里，但仍会参与 doc test 编译。
/// use doc_tests::open_file;
/// use std::io::ErrorKind;
///
/// let message = match open_file("src/main.rs") {
///     Ok(_) => String::from("opened"),
///     Err(error) if error.kind() == ErrorKind::NotFound => String::from("missing file"),
///     Err(error) => format!("other error: {error}"),
/// };
///
/// assert_eq!(message, "missing file");
/// ```
pub fn open_file(path: &str) -> Result<File, std::io::Error> {
    let file = File::open(path)?;
    Ok(file)
}

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error;

#[derive(Debug)]
struct TextStats {
    lines: usize,
    words: usize,
    unique_words: usize,
}

#[derive(Debug)]
enum AppError {
    Io(Error),
    EmptyInput,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            AppError::Io(error) => {
                // 你原来这里把底层 io error 细节也拼进输出里了，
                // 但题目要求固定文案：`failed to read input`。
                let _ = error.kind();
                write!(f, "failed to read input")
            }
            AppError::EmptyInput => write!(f, "input is empty"),
        }
    }
}

impl From<Error> for AppError {
    fn from(e: Error) -> Self {
        AppError::Io(e)
    }
}

fn read_text(path: &str) -> Result<String, AppError> {
    // 你原来手写了 match，逻辑是对的。
    // 这里改成 `?`，是为了练 `From<std::io::Error> for AppError` 的自动转换。
    Ok(std::fs::read_to_string(path)?)
}

fn normalize_word(word: &str) -> String {
    // 你原来用了 `trim()`，它只能去空白，不能去掉 `safe,` 里的逗号。
    // 题目要求去掉首尾非字母数字字符，所以这里要用 `trim_matches(...)`。
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

fn words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(normalize_word)
        .filter(|word| !word.is_empty())
        .collect()
}

fn count_words(words: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    words
        .iter()
        .for_each(|word| *map.entry(word.clone()).or_insert(0) += 1);
    map
}

fn stats(text: &str) -> Result<TextStats, AppError> {
    if text.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }
    let words = words(text);
    let counts = count_words(&words);
    Ok(TextStats {
        lines: text.lines().count(),
        words: words.len(),
        unique_words: counts.len(),
    })
}

fn top_words<F>(counts: &HashMap<String, usize>, keep: F) -> Vec<(String, usize)>
where
    F: Fn(&str, usize) -> bool,
{
    let mut result = counts
        .iter()
        .filter(|(word, count)| keep(word, **count))
        .map(|(word, count)| (word.clone(), *count))
        .collect::<Vec<_>>();

    // `sort_by` 接收一个比较用的 closure：`|a, b| ...`。
    // 这里 `a` 和 `b` 都是 `&(String, usize)`，也就是 result 里的两个元素引用。
    // Rust 访问元组字段用 `.0`、`.1`，不是 JS 那种 `[0]`、`[1]`。
    // 因为元组更像“没有字段名的 struct”；`[]` 下标主要给数组、slice、Vec 用。
    // closure 需要返回 `Ordering`，告诉排序算法 a 应该排在 b 前、后，还是相等。
    // `b.1.cmp(&a.1)` 是按 count 倒序排；如果 count 相同，
    // `.then(a.0.cmp(&b.0))` 再按 word 正序排，保证输出稳定、可预测。
    result.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    result
}

fn run(path: &str) -> Result<(), AppError> {
    let text = read_text(path)?;
    let text_stats = stats(&text)?;
    let word_list = words(&text);
    let counts = count_words(&word_list);

    // 你前面问到的那处题面歧义，实际更合理的是在 run 里传这个 closure，
    // 因为 run 负责把“读取 -> 统计 -> 筛选重复词 -> 打印”整条流程串起来。
    let repeated_words = top_words(&counts, |_, count| count >= 2);

    println!("lines: {}", text_stats.lines);
    println!("words: {}", text_stats.words);
    println!("unique words: {}", text_stats.unique_words);

    // 你原来这里直接 debug 打印了整个 Vec。
    // 题目要求固定输出 `repeated words:`，然后逐行打印 `word: count`。
    println!("repeated words:");
    // `repeated_words` 的类型是 `Vec<(String, usize)>`，里面每一项都是一个二元组。
    // `for (word, count) in repeated_words` 这里的 `(word, count)` 是解构模式：
    // 循环每取出一个 `(String, usize)`，就把第 1 个值绑定到 word，第 2 个值绑定到 count。
    for (word, count) in repeated_words {
        println!("{}: {}", word, count);
    }

    Ok(())
}

fn main() {
    match run("sample.txt") {
        // 你原来成功时打印了 `Success`，但题目不要求成功分支额外输出。
        Ok(()) => {}
        // 你原来这里是 `Error: ...`，题目要求小写：`error: ...`。
        Err(error) => println!("error: {}", error),
    }
}

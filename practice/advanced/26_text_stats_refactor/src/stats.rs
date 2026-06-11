use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct TextStats {
    pub lines: usize,
    pub words: usize,
    pub unique_words: usize,
}

// 注意这里是 crate，代表lib crate；这与main.rs 里要用 package name（text_stats_refactor::）来访问有区别。因为这是两个 crate， lib crate用内部的直接用crate，main 用 lib 的相当于外部访问了，要带上包名
pub fn stats(text: &str) -> Result<TextStats, crate::AppError> {
    if text.trim().is_empty() {
        return Err(crate::AppError::EmptyInput);
    }

    let words = crate::words::words(text);
    let unique_words = words.iter().collect::<std::collections::HashSet<_>>().len();
    Ok(TextStats {
        lines: text.lines().count(),
        words: words.len(),
        unique_words,
    })
}

pub fn top_words(
    counts: &HashMap<String, usize>,
    // REVIEW: 这里用 `impl Fn` 可以接收 closure；但本练习要求写成泛型 `F`
    // 并加 `where F: Fn(&str, usize) -> bool`，重点复习两种写法的区别。
    keep: impl Fn(&str, usize) -> bool,
) -> Vec<(String, usize)> {
    let mut list = counts
        .iter()
        .filter(|(word, count)| keep(word, **count))
        .map(|(word, count)| (word.clone(), *count))
        .collect::<Vec<_>>();

    // sort by count descending, word ascending
    // FIXME: 这里现在是 count 升序。题目要求 count 降序、word 升序。
    // list.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    // 修正后的代码如下
    list.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    list
}

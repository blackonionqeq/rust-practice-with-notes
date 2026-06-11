use std::collections::HashMap;

pub fn normalize_word(word: &str) -> String {
    word.to_string()
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

pub fn words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| normalize_word(w))
        .filter(|w| !w.is_empty())
        .collect()
}

pub fn count_words(words: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    words.iter().for_each(|word| {
        *counts.entry(word.clone()).or_default() += 1;
    });
    counts
}

use text_stats_refactor::stats::top_words;
use text_stats_refactor::words::{count_words, words};
fn main() {
    match std::fs::read_to_string("sample.txt") {
        Ok(text) => {
            // FIXME: `stats` 现在在 `stats` 模块里，不是 crate 根上的函数。
            // 可以调用 `text_stats_refactor::stats::stats(&text)`，
            // 或者在 `lib.rs` 里 re-export 这个函数。
            let stats = text_stats_refactor::stats::stats(&text);
            println!("{:?}", stats);
            const MIN_WORD_SIZE: usize = 2;
            // FIXME: `top_words` 的第一个参数应该是 `HashMap<String, usize>` 词频，
            // 不是原始文本 `&text`。需要先 `words(&text)`，再 `count_words(&words)`。
            // FIXME: closure 签名也不匹配；`top_words` 需要 `|word, count| -> bool`。
            // let top_words = text_stats_refactor::top_words(&text, |word| word.len() >= MIN_WORD_SIZE);
            // 修正后如下
            let words = words(&text);
            let count_words = count_words(&words);
            let top_words = top_words(&count_words, |_, count| count >= MIN_WORD_SIZE);
            println!("{:?}", top_words);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

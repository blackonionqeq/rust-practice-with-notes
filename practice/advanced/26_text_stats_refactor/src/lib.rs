pub mod error;
pub mod stats;
pub mod words;

pub use error::AppError;
pub use stats::{TextStats, stats, top_words};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_word() {
        let word = "Hello, World!";
        let normalized = words::normalize_word(word);
        assert_eq!(normalized, "hello, world");
    }

    #[test]
    fn test_words() {
        let text = "Hello, World!";
        let words = words::words(text);
        assert_eq!(words, vec!["hello", "world"]);
    }

    #[test]
    fn test_count_words() {
        let text = "Hello, World!";
        let words = words::words(text);
        let counts = words::count_words(&words);
        assert_eq!(counts.len(), 2);
        assert_eq!(counts.get("hello"), Some(&1));
        assert_eq!(counts.get("world"), Some(&1));
    }

    #[test]
    fn test_stats() {
        let text = "Hello, World!";
        let stats = stats(text).unwrap();
        assert_eq!(stats.words, 2);
        assert_eq!(stats.unique_words, 2);
        assert_eq!(stats.lines, 1);
    }

    #[test]
    fn test_top_words() {
        let text = "Hello, World!";

        let words = words::words(text);
        let counts = words::count_words(&words);
        let top = top_words(&counts, |_, count| count >= 1);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "hello");
        assert_eq!(top[0].1, 1);
        assert_eq!(top[1].0, "world");
        assert_eq!(top[1].1, 1);
    }
}

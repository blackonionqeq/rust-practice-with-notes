#[derive(Debug)]
pub struct Summary {
    pub lines: usize,
    pub words: usize,
}

pub fn summarize(text: &str) -> Summary {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    Summary { lines, words }
}

use std::fs::File;
use std::io::{BufRead, BufReader, Result as IoResult};
use std::path::Path;

// 注意是&Path 而不是 &PathBuf，这样允许&Path\&str\Path传入
fn count_words_in_file(path: &Path) -> IoResult<usize> {
    let file = File::open(path)?;
    // 用缓冲读，而非read_to_string，避免文件过大导致内存溢出
    let buf_reader = BufReader::new(file);
    let mut word_count = 0;
    // lines 方法依赖 BufRead trait， 所以要use
    for line in buf_reader.lines() {
        // 读取时随时会出错，先确保能读一行
        let line = line?;
        // split_whitespace 比 split 更健壮
        word_count += line.split_whitespace().count();
    }
    Ok(word_count)
}

fn main() {
    let path = Path::new("src/main.rs");
    match count_words_in_file(&path) {
        Ok(count) => println!("Word count: {}", count),
        Err(e) => println!("Error: {}", e),
    }
}

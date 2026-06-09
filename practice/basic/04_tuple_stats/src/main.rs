fn analyze(text: String) -> (String, usize, usize) {
  // 不能直接(text,text.len(),text.chars().count())，因为所有权会出问题
    let bytes = text.len();
    let chars = text.chars().count();
    (text, bytes, chars)
}

fn main() {
    let text = String::from("Rust语言练习");
    let (text, bytes, chars) = analyze(text);
    println!("Text: {}", text);
    println!("bytes: {}", bytes);
    println!("chars: {}", chars);
}

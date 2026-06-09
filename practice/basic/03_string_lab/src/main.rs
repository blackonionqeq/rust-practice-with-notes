fn main() {
    //println!("Hello, world!");
    let text = "Rust语言练习";
    let (bytes, chars, preview) = summarize(text);
    // .len()拿长度
    println!("bytes: {}", bytes);
    // .chars() 按unicode scalar value迭代 
    println!("chars: {}", chars);
    // take 取前几个char, collect::<String>()收集为String
    println!("preview: {}", preview);
}

fn summarize(text: &str) -> (usize, usize, String) {
    (
        text.len(),
        text.chars().count(),
        text.chars().take(3).collect(),
    )
}

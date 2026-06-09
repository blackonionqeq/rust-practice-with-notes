fn main() {
    //println!("Hello, world!");
    let text = String::from("rust practice");
    let (text, len) = measure(text);
    let len = format!("length: {}", len);
    println!("text: {}", text);
    println!("{}", len);
}

fn measure(text: String) -> (String, usize) {
    let len = text.len();
    (text, len)
}

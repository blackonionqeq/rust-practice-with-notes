fn make_static_label(input: String) -> &'static str {
    // 可以拆成几步：
    // let boxed: Box<str> = input.into_boxed_str();
    // let leaked: &'static str = Box::leak(boxed);
    // let shared: &'static str = leaked;
    Box::leak(input.into_boxed_str())
}

fn main() {
    // These memory leaks are intentional
    let label1 = make_static_label("Hello, World!".to_string());
    let label2 = make_static_label("Goodbye, World!".to_string());
    println!("{label1}, {label2}");
}

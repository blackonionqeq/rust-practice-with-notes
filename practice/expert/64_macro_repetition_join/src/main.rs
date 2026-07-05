macro_rules! join_words {
    // 写法挺像正则匹配的
    // 两个括号代表有返回值
    ($($word:expr),*) => {{
        // 因为是通用变量，所以放外层
        let mut result: Vec<&str> = vec![];
        // 可以展开匹配的组
        $(
            result.push($word);
        )*
        result.join("-")
    }};
}

fn main() {
    let joined = join_words!("Hello", "world", "!");
    println!("{}", joined);

    println!("{}", join_words!("rust"));
    println!("{}", join_words!("macro", "rules", "ok"));
}

#[derive(Debug)]
struct Report {
    title: String,
    tags: Vec<String>,
}

// 这里用 impl Into<String>，因为函数参数的语义是：
// “调用者给我的 title，只要能被转换成 String 就行。”
//
// 写转换实现时，通常优先实现 From：
// impl From<A> for B
// 表示“B 知道怎么从 A 构造出来”。
//
// 但函数参数这里不是在定义转换规则，而是在接收一个稍后要变成 String 的值。
// 所以用 Into<String> 更直接：
// fn f(value: impl Into<String>)
// 表示“value 可以被转换进 String”。
//
// 不能简单改成 impl From<&str>。
// impl From<&str> 约束的是“参数类型 T 可以从 &str 构造出来”，也就是 T: From<&str>；
// 但这里真正需要的是“String 可以从参数类型 T 构造出来”，也就是 String: From<T>。
//
// 如果硬要用 From 的方向表达类似能力，可以写成泛型约束：
// fn make_report<T>(title: T) -> Report
// where
//     String: From<T>,
//
// 不过作为函数参数 API，impl Into<String> 更符合调用方视角。
fn make_report(title: impl Into<String>) -> Report {
    Report {
        title: title.into(),
        tags: Vec::new(),
    }
}

// 修改用可变借用&mut
fn add_tag(report: &mut Report, tag: impl Into<String>) {
    report.tags.push(tag.into());
}

// 只读用&不可变借用
fn has_tag(report: &Report, tag: &str) -> bool {
    report.tags.iter().any(|t| t == tag)
}

// 注意返回签名用 &[String] 而非 &Vec<String>：
// 调用方只需要“连续的一段 String 视图”，不需要知道底层一定是 Vec。
//
// 这里的类型变化可以这样看：
// report.tags      的类型是 Vec<String>
// &report.tags     的类型先是 &Vec<String>
// 函数声明要求返回值是 &[String]
//
// Rust 在函数返回位置会带着“期望类型”检查最后一个表达式。
// 它发现当前表达式是 &Vec<String>，但期望的是 &[String]，
// 于是尝试做自动解引用和强制转换。
//
// Vec<T> 实现了 Deref<Target = [T]>，所以：
// &Vec<String> 可以自动转换成 &[String]
//
// 等价写法是：
// report.tags.as_slice()
// 或者：
// &report.tags[..]
//
// 这不会复制 Vec，也不会移动 String。
// 返回的 &[String] 只是借用了 report.tags 内部那段连续存储。
fn tags(report: &Report) -> &[String] {
    &report.tags
}

// 消费不使用借用而是转移所有权
fn into_title(report: Report) -> String {
    report.title
}

fn main() {
    let mut report = make_report("Rust Review");
    ["rust", "practice"]
        .iter()
        .for_each(|tag| add_tag(&mut report, *tag));
    println!("report has rust ? {}", has_tag(&report, "rust"));
    println!("{:?}", tags(&report));
    println!("title: {}", into_title(report))
}

// src/main.rs 是 binary crate 的根文件。
// 构建后对应的可执行产物通常在 target/debug/lib_modules。
// 这里通过 package/library crate 名 lib_modules 调用 src/lib.rs 导出的模块。
use lib_modules::format::format_summary;
use lib_modules::stats::summarize;

fn main() {
    // Rust 的普通字符串字面量可以直接跨多行，真实换行会成为字符串内容。
    // 行首缩进也会算进字符串；如果想少写转义，可以用 raw string：r#"..."#。
    let text = "this is
a three line
text";
    let summary = summarize(text);
    let formatted = format_summary(&summary);
    println!("{}", formatted);
}

// 这里的创建方式是： cargo new crate/text_cli --bin --name text_cli

// 这里用了隔壁text_core的东西
// 要用隔壁的东西，要改上层Cargo.toml:
// [workspace]
// members = ["crates/text_core", "crates/text_cli"]
// resolver = "2"
// 然后在这里的toml写：
// [dependencies]
// text_core = { path = "../text_core" }
use text_core::text_count;

// 使用方式：
// cargo run -p text_cli -- <text>
// -p 表示指定包，是specific package的意思
fn main() {
    let input = std::env::args().nth(1).unwrap_or("".into());
    println!("input is {input}");

    let count = text_count(&input);
    println!("The text '{}' contains {} words.", input, count);
}

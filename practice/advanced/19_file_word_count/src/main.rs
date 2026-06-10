use std::io::Error;

// 读取指定路径的文本文件。
// 返回 Result：成功时是文件内容 String，失败时是标准库提供的 IO 错误。
fn read_text(path: &str) -> Result<String, Error> {
    std::fs::read_to_string(path)
}

// &str 是借用来的字符串切片，这里只读取内容，不取得所有权。
// split_whitespace 会按空白字符分割，连续空白会自动跳过。
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// run 负责串起“读文件”和“统计单词”两个步骤。
// ? 会在出错时提前返回 Err，把错误交给 main 统一处理。
fn run(path: &str) -> Result<usize, Error> {
  // ? 让 Result<T, E> -> T
    let text = read_text(path)?;
    Ok(count_words(&text))
}

fn main() {
    // main 是程序入口，适合做“最后一层处理”：把结果打印给用户。
    // run 只负责业务逻辑并返回 Result，不在里面决定怎么展示错误。
    // 这样以后如果换成测试、命令行参数或图形界面，run 仍然可以复用。
    // match 分别处理成功和失败，避免程序在读文件失败时直接崩溃。
    match run("sample.txt") {
        Ok(count) => println!("count: {}", count),
        Err(e) => eprintln!("error: {}", e),
    }
}

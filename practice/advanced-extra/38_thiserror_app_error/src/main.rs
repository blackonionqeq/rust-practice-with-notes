use std::io::Error as IoError;
use thiserror::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Error, Debug)]
pub enum AppError {
    // error 属性用于生成Display实现
    #[error("Io error: {0}")]
    // from 用于生成From实现 + source（也就是impl Error）
    // 实现source 是为了保留错误链，以便系统错误工具能追溯
    Io(#[from] IoError),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Not found: {0}")]
    NotFound(String),
}

fn read_from_file(path: &str) -> Result<String, AppError> {
    // 直接读整个文件的版本比较简单，几行搞定：
    // let content = std::fs::read_to_string(path).map_err(AppError::Io)?;
    // for line in content.lines() {
    //     let num: i32 = line.parse::<i32>().map_err(AppError::Parse)?;
    //     println!("Parsed number: {}", num);
    // }
    // Ok(content)
    //
    // 但题目要用流
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut content = String::new();

    for line in reader.lines() {
        // AppError::Parse 会自动转换为 AppError::Io 错误
        let line = line?;
        let trimmed = line.trim();
        // 注意？只能用在返回Option或Result的地方
        let number = trimmed.parse::<i32>()?;
        println!("{number}");
        content.push_str(&line);
        content.push('\n');
    }
    Ok(content)
}

fn main() {
    let content = read_from_file("sample.txt");
    println!("Content: {:?}", content);
}

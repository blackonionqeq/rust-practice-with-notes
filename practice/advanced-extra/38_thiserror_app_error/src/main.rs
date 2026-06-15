use std::io::Error as IoError;
use thiserror::Error;

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
    let content = std::fs::read_to_string(path).map_err(AppError::Io)?;
    for line in content.lines() {
        let num: i32 = line.parse::<i32>().map_err(AppError::Parse)?;
        println!("Parsed number: {}", num);
    }
    Ok(content)
}

fn main() {
    let content = read_from_file("sample.txt");
    println!("Content: {:?}", content);
}

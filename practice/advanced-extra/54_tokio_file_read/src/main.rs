use std::{
    env,
    path::{Path, PathBuf},
};
use tokio::fs;

async fn read_file(path: &Path) -> std::io::Result<String> {
    // 注意是tokio的fs，不是标准库的
    // tokio的fs的优势是，不像标准库那样，读文件时整个线程都会被阻塞
    // 和同步函数一样可以?错误上抛
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

#[tokio::main]
async fn main() {
    let input_path = env::args().nth(1).unwrap_or("Cargo.toml".to_string());
    let path = PathBuf::from(input_path);
    let content = read_file(&path).await.unwrap_or_default();
    println!("lines: {}", content.lines().count());
    println!("words: {}", content.split_whitespace().count());
}

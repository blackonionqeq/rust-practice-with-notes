use anyhow::{Context, Result};
use std::fs;

// Result<()> 等价于 Result<T, anyhow::Error>
fn run() -> Result<()> {
    // anyhow 会把错误收集起来，我们不需要impl From
    let contents =
        fs::read_to_string("sample.txt").with_context(|| format!("Failed to read to string"))?;
    // println!("{contents}");

    let mut list: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let line = line.trim();
        let num: i32 = line
            .parse()
            .with_context(|| format!("Failed to parse int"))?;
        list.push(num);
    }

    println!("{list:?}");
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    }
}

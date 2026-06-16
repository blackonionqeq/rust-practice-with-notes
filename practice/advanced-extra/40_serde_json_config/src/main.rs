use anyhow::{Context, Result};
use std::{env::args, fs::read_to_string, io::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    // 这里不用写String如何转为PathBuf，因为serde给PathBuf实现了Deserialize，它会自己转
    // #[serde(default = "PathBuf::from")]
    input: PathBuf,

    // 不能这样写，因为default的值需要是函数
    // 可以写 default_min_count，然后下面实现个同名方法，返回Some(2)
    // #[serde(default = 2)]
    min_count: Option<usize>,

    ignore_case: bool,
}

fn run() -> Result<Config> {
    let path = PathBuf::from(args().nth(1).unwrap_or("config.json".into()));
    let content = read_to_string(&path).with_context(|| "failed to read config file")?;
    let mut config: Config =
        serde_json::from_str(&content).with_context(|| "failed to parse config")?;
    if config.min_count.is_none() {
        config.min_count = Some(2);
    }
    Ok(config)
}

fn main() {
    match run() {
        Ok(config) => println!("{config:?}"),
        Err(e) => eprintln!("{e}"),
    }
}

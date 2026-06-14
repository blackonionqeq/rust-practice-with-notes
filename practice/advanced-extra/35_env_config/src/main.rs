use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct Config {
    input: PathBuf,
    min_count: usize,
}

// wsl 或 linux 里运行可用：TEXT_INPUT=sample.txt MIN_COUNT=3 cargo run
fn main() {
    let input_path = env::args()
        .nth(1)
        // var 读取环境变量
        // var的结果是Result ，要转为Option
        .or(env::var("TEXT_INPUT").ok())
        .map(PathBuf::from);
    // 原本这么写，会让input_path仍然是Option，下面还要unwrap
    // if input_path.is_none() {
    //     eprintln!("No input path provided");
    //     return;
    // }

    // 这样if let + shadowing 来覆盖，让input_path不再是Option
    let Some(input_path) = input_path else {
        eprintln!("No input path provided");
        return;
    };

    // 原本的错误写法：map套parse，结果类型是Option套Option，导致不得不unwrap
    // let min_count = env::var("MIN_COUNT")
    //     .ok()
    //     .map(|s| s.parse::<usize>().ok())
    //     .unwrap();
    //
    let min_count = env::var("MIN_COUNT")
        .ok()
        // and_then可以在有东西时进一步处理
        // option方法，见 00_option_result_cheatsheet.md
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(2);

    let config = Config {
        input: input_path,
        min_count: min_count,
    };
    println!("input: {:?}, min_count: {}", config.input, config.min_count);
}

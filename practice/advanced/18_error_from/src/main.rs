use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;

// 这个练习的重点：把不同来源的错误统一收敛到一个 AppError。
// 这样上层函数只需要返回 Result<T, AppError>，调用方也只需要集中处理一种错误类型。
#[derive(Debug)]
enum AppError {
    // 这里保留 ParseIntError，而不是只写 Parse，是为了不丢掉底层错误信息。
    // 目前 Display 里没有把 err 打出来，所以 cargo check 会提示这个字段没有被读取；
    // 初学时可以先接受这个 warning，后面可以改成 AppError::Parse(err) => write!(f, "...{err}")。
    Parse(ParseIntError),
    // 业务规则产生的错误也放进同一个 AppError，和解析错误一起交给上层处理。
    Zero,
}

// Display 决定这个错误“展示给人看”时长什么样。
// hover 到 Display，看缺什么方法，IDE 会提示缺 fmt 方法。
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // write! 和 println! 类似，都是格式化宏；区别是 write! 不打印到终端，
        // 而是把格式化后的文本写进第一个参数。这里的 f 是 Display::fmt
        // 传进来的 Formatter，所以 write!(f, "...") 就是在定义 AppError
        // 被 {} 显示时要输出什么文本。
        match self {
            AppError::Parse(_) => write!(f, "invalid number"),
            AppError::Zero => write!(f, "cannot divide by zero"),
        }
    }
}

// 实现 From<ParseIntError> 的目的：告诉 Rust 如何把标准库的 ParseIntError
// 转换成我们自己的 AppError。这样 parse_number 里才能对 input.parse::<i32>()
// 使用 ?；? 遇到 ParseIntError 时，会自动调用 AppError::from(err) 做转换。
impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}
fn parse_number(input: &str) -> Result<i32, AppError> {
    // parse::<i32>() 原本返回 Result<i32, ParseIntError>。
    // 因为上面实现了 From<ParseIntError> for AppError，这里的 ? 可以自动完成错误类型转换。
    Ok(input.parse::<i32>()?)
}
fn divide(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        Err(AppError::Zero)
    } else {
        Ok(a / b)
    }
}
fn parse_and_divide(a: &str, b: &str) -> Result<i32, AppError> {
    divide(parse_number(a)?, parse_number(b)?)
}
fn print_result(a: &str, b: &str) {
    match parse_and_divide(a, b) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
fn main() {
    print_result("10", "2");
    print_result("10", "0");
    print_result("abc", "2");
}

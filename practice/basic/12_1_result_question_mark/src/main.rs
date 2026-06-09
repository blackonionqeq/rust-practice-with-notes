fn parse_number(input: &str) -> Result<i32, String> {
    // if let 替代 match
    if let Ok(res) = input.parse::<i32>() {
        Ok(res)
    } else {
        Err(String::from("invalid number"))
    }
}
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
fn parse_and_divide(a: &str, b: &str) -> Result<i32, String> {
    // ?会错误就往上抛，所及即使被Ok包裹也没关系
    Ok(divide(parse_number(a)?, parse_number(b)?)?)
}
fn print_result(a: &str, b: &str) {
    match parse_and_divide(a, b) {
        Ok(value) => println!("result: {}", value),
        Err(msg) => println!("error: {}", msg),
    }
}
fn main() {
    print_result("10", "2");
    print_result("10", "0");
    print_result("abc", "2");
}

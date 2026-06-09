/**
*   功能：
1. 尝试将字符串解析为 i32
2. 如果成功，返回 Some(数字 * 2)
3. 如果失败，返回 None

约束
- parse_and_double 内不使用 match 或 if let
- 必须使用 Option 的方法链
*/
fn parse_and_double(s: &str) -> Option<i32> {
    //let num = s.parse::<i32>().ok()?;
    //Some(num * 2)
    s.parse::<i32>().ok().map(|n| n * 2)
}

fn print_result(s: &str) {
    if let Some(val) = parse_and_double(s) {
        println!("{}", val);
    } else {
        println!("None");
    }
}

fn main() {
    print_result("42");
    print_result("abc");
    print_result("0");
    //    println!("Hello, world!");
}

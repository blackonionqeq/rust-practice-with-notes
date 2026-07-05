// ide不好看宏的类型
// 在vscode 可以考虑用展开结果调试宏：rust analyzer: expand macro recursively
// 或 cargo expand 展开(要另外装cargo-expand的依赖)
// 一般可先通过方法实现，然后丢到宏里
macro_rules! measure_len {
    ($input:expr) => {{
        let input_str = $input.to_string();
        input_str.len()
    }};
}

fn main() {
    let value = String::from("rust");
    println!("{}", measure_len!(value.clone()));
    let tmp = 123;
    println!("{}", measure_len!(tmp));
}

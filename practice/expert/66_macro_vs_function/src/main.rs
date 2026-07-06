macro_rules! to_kv_line {
    ($key:expr, $value:expr) => {
        format!("{}: {}", $key, $value)
    };
}

fn to_kv_line(k: &str, v: &str) -> String {
    format!("{}: {}", k, v)
}

fn main() {
    // Function vs macro:
    // 能用函数，优先用函数；因为宏的可读性、可调试性、维护成本略高
    // 比如不定长参数，再考虑用宏
    let result1 = to_kv_line("key1", "value1");
    let result2 = to_kv_line!("key2", "value2");
    println!("{result1}");
    println!("{result2}");
}

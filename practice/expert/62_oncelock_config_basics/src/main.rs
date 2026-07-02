use std::collections::HashMap;
use std::sync::OnceLock;

// 这里的类型声明不加'static也能跑，因为编译器发现这个变量的位置，会自己加'static，但手动加上更好，能让使用者一眼看清楚
static CONFIG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

fn init_config() -> Result<(), &'static str> {
    // 注意from的格式：array套元组
    let map = HashMap::from([("mode", "dev"), ("port", "8080")]);
    // OnceLock的api其一：set；第一次正常ok，第二次和以后调用会报错
    CONFIG.set(map).map_err(|_| "config already initialized")
}

fn get_config(key: &str) -> Option<&'static str> {
    //OnceLock的api其二：get，注意无参数，返回是Option；
    // 注意copied不可省略
    CONFIG.get().and_then(|map| map.get(key).copied())
}

fn main() {
    match init_config() {
        Err(e) => eprintln!("Error1: {e:?}"),
        _ => {}
    }
    println!(
        "mode is: {}, port is: {}",
        get_config("mode").unwrap_or(""),
        get_config("port").unwrap_or("")
    );
    if let Err(e) = init_config() {
        println!("Error2: {e:?}");
    }
}

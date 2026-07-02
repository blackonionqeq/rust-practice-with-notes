// 宏名，注意声明时不用加!
macro_rules! say_hello {
    // $name 是匹配到的语法，不是普通变量
    ($name:expr) => {
        // 宏可以套宏
        println!("Hello, {}!", $name);
    };
}

fn main() {
    say_hello!("world");

    say_hello!(String::from("macro"));
}

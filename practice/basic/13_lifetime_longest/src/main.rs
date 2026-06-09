// 这是告诉编译器，返回值的生命周期不会比入参短
fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}
fn print_longest(first: &str, second: &str) {
    println!("longest: {}", longest(first, second));
}

fn main() {
    //    println!("");
    print_longest(&String::from("rust"), &String::from("ownership"));
    let short = "abc";
    let long = "abcdef";
    print_longest(short, long);
}

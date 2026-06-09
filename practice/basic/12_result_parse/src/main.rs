fn parse_age(input: &str) -> Result<u32, String> {
    match input.parse::<u32>() {
        Ok(age) => Ok(age),
        Err(..) => Err(String::from("invalid age")),
    }
}
fn print_age(input: &str) {
    match parse_age(input) {
        Ok(age) => println!("age: {}", age),
        Err(msg) => println!("error: {}", msg),
    }
}
fn main() {
    let targets: [&str; 3] = ["18", "abc", "0"];
    for target in targets {
        print_age(target)
    }
}

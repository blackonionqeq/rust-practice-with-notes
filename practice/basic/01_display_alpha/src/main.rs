fn main() {
    // println!("Hello, world!");
    display_alpha();
}

fn display_alpha() {
    let mut count = 0;
    for i in 'a'..='z' {
        println!("{}", i);
        count += 1;
    }
    println!("total: {}", count);
}

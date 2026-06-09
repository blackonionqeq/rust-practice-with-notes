fn describe_number(n: i32) {
    //println!("{}", str);
    if n > 0 {
        println!("{} is positive", n)
    } else if n == 0 {
        println!("{} is zero", n)
    } else {
        println!("{} is negative", n)
    }
}
fn count_down(start: i32) {
    let mut s = start;
    while s >= 1 {
        println!("{}", s);
        s -= 1;
    }
    println!("done")
}
fn print_odd_numbers() {
    for i in 1..=10 {
        // 注意不能写成 i %= 2
        if i % 2 == 0 {
            continue;
        } else {
            println!("{}", i)
        }
    }
}
fn main() {
    describe_number(7);
    describe_number(0);
    describe_number(-3);
    count_down(3);
    print_odd_numbers();
    //    println!("Hello, world!");
}

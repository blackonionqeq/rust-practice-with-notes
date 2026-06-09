fn grade(score: i32) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "invalid",
    }
}
fn describe_grade(s: Option<i32>) {
    match s {
        // 注意这里用Some来缩窄类型
        Some(val) => println!("score: {}, grade: {}", val, grade(val)),
        None => println!("no score"),
    }
}
fn main() {
    let arr: [Option<i32>; 5] = [Some(95), Some(82), Some(58), Some(120), None];
    for score in arr {
        describe_grade(score)
    }
    //println!("Hello, world!");
}

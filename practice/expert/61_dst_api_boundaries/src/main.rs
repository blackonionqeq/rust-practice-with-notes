fn first_word(input: &str) -> &str {
    input.split_whitespace().next().unwrap_or("")
}

fn sum_slice(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn main() {
    // &String能在需要&str的场景下，被自动转换，是因为实现了Deref，详情见笔记
    println!(
        "Call first_word with String: {}",
        first_word(&String::from("Hello World"))
    );
    println!(
        "Call first_word with string literal: {}",
        first_word("Hello World")
    );

    println!(
        "Call sum_slice with vec: {}",
        sum_slice(&vec![1, 2, 3, 4, 5])
    );
    println!(
        "Call sum_slice with array slice: {}",
        sum_slice(&[1, 2, 3, 4, 5])
    );
}

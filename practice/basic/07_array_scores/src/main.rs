fn print_scores(arr: [i32; 5]) {
    println!("{}", arr[0]);
    println!("{}", arr[4]);
    println!("len: {}", arr.len());
    for i in arr {
        println!("{}", i);
    }
}

fn sum_scores(arr: [i32; 5]) -> i32 {
    let mut sum: i32 = 0;
    for i in arr {
        sum += i;
    }
    sum
}

fn main() {
    let arr: [i32; 5] = [80, 92, 75, 88, 100];
    print_scores(arr);
    println!("total: {}", sum_scores(arr));
    //    println!("Hello, world!");
}

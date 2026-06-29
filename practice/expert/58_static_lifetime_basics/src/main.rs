fn takes_static_ref(s: &'static str) -> usize {
    s.len()
}

fn takes_static_bound<T: 'static>(value: T) -> T {
    value
}

fn main() {
    let s = "hello";
    let result = takes_static_ref(s);
    println!("{result}");

    let value = String::from(s);
    let result = takes_static_bound(value);
    println!("{result}");

    // &String is not 'static, so this will not compile
    // &String 可自动deref为 &str，但不会自动变成&'static str
    // 因为 从&String 借用出来本身意味着是借用，会受借用的生命周期限制
    // takes_static_ref(&value);
    // 而 &'static str （例如上面的hello） 是静态生命周期（放在只读静态区，能活到程序结束），不受借用的生命周期限制
}

#[derive(Debug)]
struct Counter {
    value: i32,
}

impl Counter {
    // impl里没self的叫关联函数
    fn new() -> Counter {
        Counter { value: 0 }
    }
    // 有self的叫方法
    fn current(&self) -> i32 {
        self.value
    }
    fn increment(&mut self) {
        self.value += 1;
    }
    fn add(&mut self, amount: i32) {
        self.value += amount;
    }
    fn reset(&mut self) {
        self.value = 0;
    }
}

fn main() {
    // 要加mut 不然成员无法被变更
    // 关联函数用结构体::调用
    let mut counter = Counter::new();
    // 方法用.访问
    println!("current: {}", counter.current());
    counter.increment();
    counter.increment();
    counter.add(5);
    println!("current: {}", counter.current());
    counter.reset();
    println!("{:?}", counter);
    //    println!("Hello, world!");
}

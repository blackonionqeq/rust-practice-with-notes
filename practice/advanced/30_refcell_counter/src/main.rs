use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
  value: RefCell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: RefCell::new(0),
        }
    }

    fn increment(&self) {
        // mut 不可省略
        // borrow_mut 返回 RefMut，会进行运行期检查
        // 而mut 是编译器检查，不加但下一行改，编译器会报错
        // 另外 `borrow_mut()` 拿到的是“可变借用守卫”，不是里面的值本身；前面的 `*` 是对这个守卫解引用，才能改到真正的内部值
        let mut value = self.value.borrow_mut();
        *value += 1;
    }

    fn add(&self, amount: i32) {
        *self.value.borrow_mut() += amount;
    }

    fn current(&self) -> i32 {
        *self.value.borrow()
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment();
    counter.add(5);
    println!("Current value: {}", counter.current());
    println!("{:?}", counter);
}

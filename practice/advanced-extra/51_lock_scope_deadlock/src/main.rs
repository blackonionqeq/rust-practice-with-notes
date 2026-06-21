use std::{
    ops::AddAssign,
    sync::{Arc, Mutex},
    thread::spawn,
};

type Shared<R> = Arc<Mutex<R>>;

// 为了避免死锁，需要以固定的顺序取相应的数据
// 考虑如下场景：
// 线程a和b，都尝试获取counter a和b
// 当a先获取counter a，还未获取counter b；而线程b先获取counter b，未获取counter a
// 则彼此依赖导致死锁
fn safe_fn<R: AddAssign + Copy>(state_a: Shared<R>, state_b: Shared<R>, by: R) {
    // 一般生产环境会unwrap，因为if let Ok会吞掉poison的情况，让错误不可见
    if let Ok(mut counter) = state_a.lock() {
        *counter += by;
        // 这里用锁a且锁b来模拟死锁的场景，实际可以用下面注释的代码，不在相同作用域持有，也能避免死锁
        if let Ok(mut counter) = state_b.lock() {
            *counter += by;
        }
    }
    // 这种写法也能避免死锁，但两次操作不原子
    // if let Ok(mut counter) = state_b.lock() {
    //     *counter += by;
    // }
}

fn main() {
    let counter_a = Arc::new(Mutex::new(0.0));
    let counter_b = Arc::new(Mutex::new(0.0));

    // 也可以collect后into_iter，但range也一样可以for in读取，所以都行
    let handles = (0..5).map(|_| {
        // 注意要在这里clone，而不是下面的spawn里，否则会报错
        // 这是闭包+spawn生命周期的经典坑点：
        // 闭包不能借用任何绑定在当前栈帧上的局部变量，因为线程完全可能比创建它的函数活得更久
        let counter_a = Arc::clone(&counter_a);
        let counter_b = Arc::clone(&counter_b);
        spawn(move || {
            safe_fn(counter_a, counter_b, 1.);
        })
    });
    for handle in handles {
        handle.join().unwrap()
    }
    println!("counterA: {counter_a:?}, counterB: {counter_b:?}");
    assert_eq!(*counter_a.lock().unwrap(), 5.)
}

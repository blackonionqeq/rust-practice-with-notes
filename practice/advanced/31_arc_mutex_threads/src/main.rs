use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

fn run_counter(thread_count: usize, increments_per_thread: usize) -> i32 {
    // Arc 是原子引用计数，多个线程可以共享同一个Arc，通过clone增加引用计数
    // Mutex 处理的是互斥锁，保证同一时间只有一个线程可以访问共享资源
    // 不能只用Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..thread_count {
        // 在这里clone，而不是直接在线程里用counter，是因为每个线程都需要拥有一个 Arc 句柄
        // Arc::clone 只增加引用计数，不复制内部的 Mutex<i32>
        let counter = Arc::clone(&counter);

        // spawn闭包内的返回值类型是JoinHandle<T>, T是闭包内的返回值类型
        // move关键字将counter移动到闭包内，避免多个线程共享同一个counter
        let handle: JoinHandle<_> = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                // lock 会阻塞当前线程，直到获取到锁
                // 如果lock失败，会返回Err，会panic，后续步骤不会执行
                // 成功时拿到的是 MutexGuard，需要解引用后使用，使用后离开作用域时，会自动Drop
                // poisoned 指的是：某个线程在持有锁期间 panic，Mutex 被标记为可能处于不一致状态，而不是获取锁失败；获取锁失败会阻塞到拿到锁
                let mut count = counter.lock().expect("mutex poisoned");
                *count += 1;
            }

            // 另一种方式：只锁一次，循环内多次递增
            // let mut count = counter.lock().expect("mutex poisoned");
            // for _ in 0..increments_per_thread {
            //     *count += 1;
            // }
        });

        handles.push(handle);
    }

    for handle in handles {
        // join 会阻塞当前线程，直到子线程执行完毕
        // 如果子线程panic，会返回Err，这里会直接panic
        handle.join().expect("thread panicked")
    }

    let count = counter.lock().expect("mutex poisoned");
    *count
}

fn main() {
    let result = run_counter(4, 1000);
    println!("count: {}", result);
}

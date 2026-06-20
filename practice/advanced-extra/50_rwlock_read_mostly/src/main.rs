use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread::{JoinHandle, spawn};
// use rand::random;

type LockType = RwLock<HashMap<String, usize>>;
type MapType = Arc<LockType>;

// spawn 创建的线程要求闭包中捕获的变量具有 'static 生命周期，因为新线程可能比调用函数活得更久
// 如果接收的是引用 &LockType 和 &str，这些引用的生命周期仅限于函数体内，不能逃逸到 spawn 的闭包中。
// 注意整个 MapType 丢进来，而不是 LockType, Arc 可以直接共享，且因为实现了Deref，所以可直接用内层的方法
fn spawn_read_thread(map: MapType, key: String) -> JoinHandle<()> {
    // 声明 move 是因为 闭包中捕获的变量需要在新线程中使用，而不是在调用线程中使用
    // spawn 要求使用的变量是'static 生命周期，因为新线程可能比调用线程活得更久
    // 进而倒退spawn_read_thread提供有所有权的变量
    spawn(move || {
        if let Ok(map) = map.read() {
            let value = {
                map.get(&key).copied()
            };
            if let Some(value) = value {
                println!("read: {key} -> {value}");
            }
        }
    })
}

fn main() {
    let map = Arc::new(RwLock::new(HashMap::<String, usize>::new()));

    let strs = ["foo", "bar", "baz", "qux"].map(|str| str.to_string());
    for str in &strs {
        map.write().unwrap().insert(str.clone(), str.len());
    }

    println!("current size: {}", map.read().unwrap().len());
    let read_threads = strs
        .iter()
        // 注意 Arc::clone, clone 的是整个Lock+HashMap
        .map(|str| spawn_read_thread(Arc::clone(&map), str.clone()))
        .collect::<Vec<_>>();

    for thread in read_threads {
        match thread.join() {
            Ok(_) => {}
            Err(e) => panic!("{:?}", e),
        }
    }
    map.write().unwrap().insert("test".into(), 42);
    println!("current size: {}", map.read().unwrap().len());
}

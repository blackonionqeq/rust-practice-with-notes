# 48 Thread Join Handles - 补充笔记

## 背景知识

### 用 thread::spawn 启动线程

`std::thread::spawn` 接受一个闭包，在新线程里运行它，并返回一个 `JoinHandle`：

```rust
use std::thread;

let handle = thread::spawn(|| {
    // 这个闭包在新线程里执行
    1 + 2
});
```

`spawn` 返回的 `JoinHandle<T>` 里的 `T` 是闭包的返回值类型。上面这个 handle 的类型是 `JoinHandle<i32>`。

### move 把所有权交给线程

默认情况下闭包只能借用外部变量，但线程可能活得比当前函数久，借用会不安全。所以闭包要使用外部变量时，通常需要 `move`，把变量的所有权**移动**进闭包：

```rust
let numbers = vec![1, 2, 3, 4];

let handle = thread::spawn(move || {
    // numbers 的所有权移动进了这个线程
    numbers.iter().sum::<i32>()
});
// 这里不能再使用 numbers 了
```

本题里每个线程负责一段数字的部分和，用 `move` 把属于它的那一段数据搬进去，是最干净的做法。

### JoinHandle::join 等待线程结束并取回返回值

`handle.join()` 会阻塞当前线程，直到子线程跑完，返回 `Result<T, Box<dyn Any + Send>>`：

```rust
match handle.join() {
    Ok(partial_sum) => println!("部分和: {}", partial_sum),
    Err(_panic_payload) => println!("子线程 panic 了"),
}
```

注意它返回的是 `Result`：

- `Ok(value)` —— 子线程正常返回，`value` 就是闭包的返回值。
- `Err(_)` —— 子线程 panic 了，里面装的是 panic 的信息。

所以 `join` 不能直接 `unwrap`，否则一个子线程 panic 就会让主线程也跟着 panic。应该用 `match` 处理。

### 收集多个线程的结果

把每个 handle 的返回值收集起来再相加：

```rust
let handles: Vec<_> = (0..3).map(|i| {
    let chunk = chunks[i].clone(); // 每个线程拿到自己的数据
    thread::spawn(move || chunk.iter().sum::<i32>())
}).collect();

let mut total = 0;
for handle in handles {
    if let Ok(partial) = handle.join() {
        total += partial;
    }
}
```

### 为什么这道题不用共享可变状态

每个线程只读自己 `move` 进来的数据、算出独立结果，再由主线程汇总。这种"分而治之、最后合并"的模式不需要 `Arc<Mutex<_>>`，因为线程之间没有任何共享数据。能用返回值解决的问题，就别上锁。

---

## 本题覆盖

Practice spawning threads and collecting their return values.

## 需要重点理解

- `thread::spawn` 返回 `JoinHandle<T>`，`T` 是闭包的返回类型。
- 用 `move` 把数据所有权转移进线程闭包，避免借用生命周期问题。
- `join()` 返回 `Result`，子线程 panic 时是 `Err`，不能无脑 `unwrap`。
- 每个线程只处理独立数据时，不需要共享可变状态。

## 常见坑

- 忘记 `move`，编译器报"闭包可能比借用活得久"的错误。
- 对 `join()` 的返回值用 `unwrap`，子线程一旦 panic 主线程也跟着崩。
- 用 `Arc<Mutex<Vec<_>>>` 共享一个可变数组去汇总——其实用返回值更简单。
- 启动了线程却忘记 `join`，程序可能在子线程跑完前就退出。

## 回看问题

- 每个线程拿到的是"自己的数据"还是"共享的数据"？所有权是怎么转移的？
- 如果某个子线程 panic，主线程会得到什么？怎么处理更友好？
- 这道题的数据划分方式，能不能直接套用到并行计算更大的任务上？

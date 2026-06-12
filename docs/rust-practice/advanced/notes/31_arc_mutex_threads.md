# 31 Arc Mutex Threads - 补充笔记

## 本题覆盖

- `Arc<T>`：跨线程共享所有权。
- `Mutex<T>`：跨线程安全地修改同一个值。
- `Arc<Mutex<T>>`：最常见的“多线程共享可变状态”组合。
- `std::thread::spawn`：创建线程。
- `JoinHandle::join`：等待线程结束并拿到线程返回值或 panic 信息。

## 核心结论

如果多个线程都要修改同一个计数器，通常写成：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));

let mut handles = Vec::new();

for _ in 0..4 {
    let counter = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            let mut count = counter.lock().expect("mutex poisoned");
            *count += 1;
        }
    });

    handles.push(handle);
}

for handle in handles {
    handle.join().expect("thread panicked");
}

let count = counter.lock().expect("mutex poisoned");
println!("count: {}", *count);
```

这里分工很明确：

- `Arc` 解决“多个线程都要拥有同一个值”的问题。
- `Mutex` 解决“多个线程不能同时修改同一个值”的问题。
- `thread::spawn` 创建线程。
- `move || { ... }` 把当前线程需要的数据移动进新线程。
- `join` 等待线程跑完。

## `Arc<T>` 常用 API

### `Arc::new(value)`

创建一个带原子引用计数的共享指针：

```rust
use std::sync::Arc;

let numbers = Arc::new(vec![1, 2, 3]);
```

`Arc<T>` 类似 `Rc<T>`，但 `Arc` 的引用计数是原子操作，因此可以安全地跨线程共享。

### `Arc::clone(&arc)`

增加引用计数，得到另一个指向同一份数据的 `Arc`：

```rust
let a = Arc::new(String::from("hello"));
let b = Arc::clone(&a);
```

习惯上推荐写 `Arc::clone(&a)`，而不是 `a.clone()`，因为这样一眼能看出这是在克隆智能指针，不是在深拷贝内部数据。

注意：`Arc::clone` 不会复制内部的 `T`，只是增加引用计数。

### `Arc::strong_count(&arc)`

查看当前强引用数量，主要用于调试或学习：

```rust
let a = Arc::new(10);
let b = Arc::clone(&a);

assert_eq!(Arc::strong_count(&a), 2);
```

练习里一般不需要用它。

### `Arc` 本身不提供可变访问

下面这种思路是错的：

```rust
// 伪代码：Arc<i32> 不能直接被多个线程修改
let counter = Arc::new(0);
```

`Arc<T>` 只解决共享所有权，不解决“如何安全修改”。

要修改共享数据，需要把内部值放进同步类型里，例如：

```rust
Arc<Mutex<i32>>
```

## `Mutex<T>` 常用 API

### `Mutex::new(value)`

创建一个互斥锁保护的值：

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);
```

单线程里可以单独使用 `Mutex<T>`，跨线程共享时通常再包一层 `Arc`：

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
```

### `mutex.lock()`

加锁并获取内部值的访问权：

```rust
let mut count = counter.lock().expect("mutex poisoned");
*count += 1;
```

`lock()` 的返回类型大致可以理解为：

```rust
Result<MutexGuard<'_, T>, PoisonError<MutexGuard<'_, T>>>
```

也就是说：

- 成功时拿到 `MutexGuard`。
- 失败时说明这个 mutex 被 poison 了。

### `MutexGuard`

`lock()` 成功后得到的是 `MutexGuard`，不是直接得到内部值。

```rust
let mut guard = counter.lock().expect("mutex poisoned");
*guard += 1;
```

`MutexGuard` 做两件事：

1. 通过 `Deref` / `DerefMut` 让你像使用 `T` 一样使用内部值。
2. 在离开作用域时自动释放锁。

所以不用手动 `unlock`：

```rust
{
    let mut count = counter.lock().expect("mutex poisoned");
    *count += 1;
} // count 在这里 drop，锁自动释放
```

这里如果 `lock()` 返回 `Err`，`expect("mutex poisoned")` 会直接 panic，后面的 `*count += 1` 不会执行。只有 `lock()` 成功拿到 `MutexGuard` 时，`count` 才会存在。

### 控制锁的作用域

锁持有时间越短越好。

可以这样写，让每次自增只持有很短的锁：

```rust
for _ in 0..1000 {
    let mut count = counter.lock().expect("mutex poisoned");
    *count += 1;
}
```

也可以这样写，只锁一次，然后连续自增：

```rust
let mut count = counter.lock().expect("mutex poisoned");
for _ in 0..1000 {
    *count += 1;
}
```

这两种都能得到正确结果，但语义不同：

- 每次循环加锁：更能练习并发竞争，锁粒度更小，但开销更大。
- 每个线程只加一次锁：开销小，但一个线程会连续占着锁，其他线程等待更久。

本题要求只是最终计数正确，两种都可以。更贴近“多个线程竞争同一个计数器”的写法是每次自增时加锁。

### `mutex.into_inner()`

如果你已经拥有整个 `Mutex<T>`，可以取出内部值：

```rust
use std::sync::Mutex;

let mutex = Mutex::new(5);
let value = mutex.into_inner().expect("mutex poisoned");

assert_eq!(value, 5);
```

但对于 `Arc<Mutex<T>>`，通常不能直接 `into_inner`，因为内部值可能还有其他 `Arc` 引用。练习里更简单的方式是最后再 `lock()` 一次读出结果。

## `std::thread` 常用 API

### `thread::spawn`

创建新线程：

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hello from child thread");
});
```

`spawn` 返回 `JoinHandle<T>`。

闭包返回什么，`JoinHandle` 的类型参数就是什么：

```rust
let handle = thread::spawn(|| {
    1 + 2
});

let result = handle.join().expect("thread panicked");
assert_eq!(result, 3);
```

### 为什么经常要写 `move ||`

线程可能比当前函数活得更久，所以 `thread::spawn` 要求闭包捕获的数据满足 `'static` 约束。

通常写：

```rust
let data = Arc::new(Mutex::new(0));
let data_for_thread = Arc::clone(&data);

let handle = thread::spawn(move || {
    let mut value = data_for_thread.lock().expect("mutex poisoned");
    *value += 1;
});
```

`move` 表示把 `data_for_thread` 移动进新线程。这里移动的是 `Arc` 指针本身，不是把内部数据复制一份。

### `JoinHandle::join`

等待线程结束：

```rust
handle.join().expect("thread panicked");
```

`join()` 返回 `Result<T, Box<dyn Any + Send + 'static>>`，可以简单理解为：

- `Ok(value)`：线程正常结束，并返回闭包的返回值。
- `Err(_)`：线程 panic 了。

如果不 `join`：

- 主线程可能先结束，程序直接退出。
- 你无法确认子线程是否完成。
- 你拿不到子线程 panic 信息。

## 本题函数骨架

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn run_counter(thread_count: usize, increments_per_thread: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut count = counter.lock().expect("mutex poisoned");
                *count += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("thread panicked");
    }

    let count = counter.lock().expect("mutex poisoned");
    *count
}
```

注意最后一行：

```rust
*count
```

`count` 是 `MutexGuard<i32>`，`*count` 才是内部的 `i32`。

因为 `i32` 实现了 `Copy`，所以可以直接复制出来作为返回值。

## Poison 是什么

如果某个线程持有锁时 panic，这个 `Mutex` 会被标记为 poisoned。

之后再调用 `lock()` 会返回 `Err`，提醒你：内部数据可能处于不完整或不一致状态。

本练习里可以使用：

```rust
counter.lock().expect("mutex poisoned")
```

这比单纯 `unwrap()` 更清楚。

如果实际项目要恢复，可以显式处理：

```rust
let mut count = match counter.lock() {
    Ok(guard) => guard,
    Err(poisoned) => poisoned.into_inner(),
};

*count += 1;
```

不过这表示你接受“即使发生过 panic，也继续使用内部数据”，真实项目里要谨慎。

## `Arc<Mutex<T>>` 和 `Rc<RefCell<T>>` 的区别

| 场景 | 常用组合 | 是否多线程 | 借用/同步检查 |
| --- | --- | --- | --- |
| 单线程共享可变状态 | `Rc<RefCell<T>>` | 否 | 运行时借用检查 |
| 多线程共享可变状态 | `Arc<Mutex<T>>` | 是 | 互斥锁同步 |

简单记：

- 单线程：`Rc<RefCell<T>>`
- 多线程：`Arc<Mutex<T>>`

## 常见坑

### 1. 在线程循环里直接移动原始 `Arc`

错误思路：

```rust
let counter = Arc::new(Mutex::new(0));

for _ in 0..4 {
    thread::spawn(move || {
        // 第一次循环就把 counter move 进线程了
        let mut count = counter.lock().expect("mutex poisoned");
        *count += 1;
    });
}
```

应该每个线程 clone 一个 `Arc`：

```rust
for _ in 0..4 {
    let counter = Arc::clone(&counter);

    thread::spawn(move || {
        let mut count = counter.lock().expect("mutex poisoned");
        *count += 1;
    });
}
```

### 2. 忘记 `move`

线程闭包通常需要 `move`：

```rust
thread::spawn(move || {
    // 使用移动进来的 Arc
});
```

否则编译器可能提示闭包借用了当前栈上的变量，而新线程不一定安全。

### 3. 忘记解引用 `MutexGuard`

错误：

```rust
let mut count = counter.lock().expect("mutex poisoned");
count += 1;
```

正确：

```rust
let mut count = counter.lock().expect("mutex poisoned");
*count += 1;
```

### 4. 忘记保存和 join 线程

```rust
let mut handles = Vec::new();

for _ in 0..4 {
    let handle = thread::spawn(|| {
        // work
    });

    handles.push(handle);
}

for handle in handles {
    handle.join().expect("thread panicked");
}
```

### 5. 持锁时做太多事情

尽量不要在持有 `MutexGuard` 时做耗时操作，例如复杂 IO、网络请求、长时间计算。

锁住的代码越多，其他线程等待越久。

## 回看问题

- 为什么多线程用 `Arc` 而不是 `Rc`？
- `Arc<T>` 解决的是共享所有权，还是共享可变性？
- `Mutex<T>` 解决的是所有权问题，还是同步问题？
- `lock()` 为什么返回 `Result`？
- `MutexGuard` 什么时候释放锁？
- 为什么线程闭包经常要写 `move`？
- 为什么必须保存 `JoinHandle` 并调用 `join()`？

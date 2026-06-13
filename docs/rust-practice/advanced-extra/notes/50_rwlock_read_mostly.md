# 50 RwLock Read Mostly - 补充笔记

## 背景知识

### Mutex 和 RwLock 的区别

- `Mutex<T>`：同一时刻只允许**一个**线程访问数据（无论读还是写）。拿到锁的人独占。
- `RwLock<T>`：允许**多个读者同时**读，但**写者独占**。读多写少的场景下吞吐量更高。

```rust
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

let data = Arc::new(RwLock::new(HashMap::<String, usize>::new()));
```

`Arc` 提供线程安全的引用计数（让多个线程共享所有权），`RwLock` 提供内部可变性。组合起来就是 `Arc<RwLock<T>>`——线程间共享、可读可写。

### 读锁和写锁

```rust
// 读：可以多个 reader 同时持有 read guard
{
    let map = data.read().unwrap();   // RwLockReadGuard
    if let Some(v) = map.get("apple") {
        println!("apple = {}", v);
    }
}   // guard 在这里 drop，释放读锁

// 写：write 是独占的，会等所有 reader 退出
{
    let mut map = data.write().unwrap();  // RwLockWriteGuard
    map.insert("apple".to_string(), 3);
}   // 释放写锁
```

`read()` / `write()` 返回 `Result`，因为锁可能被"毒化"（poisoned）——持有锁的线程 panic 了，锁就处于不确定状态。`unwrap` 在这里要小心，最好用 `match` 处理。

### 锁要尽早释放

guard（`RwLockReadGuard` / `RwLockWriteGuard`）在 drop 时才释放锁。所以**把锁的作用域缩到最小**：用 `{}` 包起来，取值后马上让 guard 离开作用域：

```rust
// 好：锁只持有很短的时间
let value = {
    let map = data.read().unwrap();
    map.get("apple").copied()
};   // 读锁在这里释放
// 后续的耗时操作不再持有锁
do_something_slow(value);
```

```rust
// 差：持锁期间做了不相关的工作，挡住了其他线程
let mut map = data.write().unwrap();
map.insert("apple".to_string(), 3);
do_something_slow();   // 还拿着写锁！其他 reader 全被阻塞
```

### 什么时候用 RwLock，什么时候用 Mutex

- **读远多于写** → `RwLock`：多个 reader 并行，吞吐高。
- **读写差不多，或写很频繁** → `Mutex`：`RwLock` 本身比 `Mutex` 重，写多读少时反而更慢。
- 数据结构本身已经有细粒度并发控制（如 `DashMap`）→ 直接用那个，别再包一层锁。

---

## 本题覆盖

Practice `Arc<RwLock<T>>` for read-heavy shared state.

## 需要重点理解

- `RwLock` 允许多读单写，读多写少时比 `Mutex` 更高效。
- `Arc<RwLock<T>>` 是"线程间共享 + 内部可变"的常见组合。
- 锁 guard 在 drop 时释放，作用域越小，并发越好。
- `read()` / `write()` 返回 `Result`，要处理毒化锁，不能无脑 `unwrap`。

## 常见坑

- 持锁期间做耗时或 IO 操作，把其他线程全堵死。
- 对 `read()` / `write()` 的返回值 `unwrap`，遇到毒化锁时 panic。
- 读多写少却用了 `Mutex`，白白损失并发度。
- 写者在持有写锁时又去请求读锁，造成死锁。

## 回看问题

- 你的 reader 拿到锁之后，有没有立刻把需要的值 copy 出来再放锁？
- 这道题是读多写少吗？用 `RwLock` 相比 `Mutex` 带来了什么好处？
- 如果持有写锁的线程 panic 了，后续 `read()` 会得到什么？

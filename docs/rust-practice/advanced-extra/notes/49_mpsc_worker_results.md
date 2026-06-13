# 49 MPSC Worker Results - 补充笔记

## 背景知识

### channel 是什么

`std::sync::mpsc` 提供一个"多生产者、单消费者"（Multi-Producer, Single-Consumer）的通道。一头送数据，一头收数据，数据在线程间安全传递：

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel(); // tx 发送端, rx 接收端
```

`channel()` 返回一个元组 `(Sender<T>, Receiver<T>)`。

### Sender 可以克隆，Receiver 不行

通道是"多生产者、单消费者"：发送端可以有多个，但接收端只能有一个。给每个 worker 线程一个发送端，只要 clone 一下：

```rust
let (tx, rx) = mpsc::channel();

for i in 0..3 {
    let worker_tx = tx.clone(); // 每个 worker 拿一个副本
    thread::spawn(move || {
        let result = compute(i);
        worker_tx.send(result).unwrap();
    });
}
// 原始的 tx 还留在主线程
```

`Receiver` 没有 `Clone`，因为它设计上只能被一个线程持有。

### send 和 recv 都返回 Result

发送和接收都可能失败，所以都返回 `Result`，不要 `unwrap`：

```rust
// 发送：如果所有接收端都被 drop 了，send 会失败
match worker_tx.send(result) {
    Ok(()) => {}
    Err(_) => println!("接收端已关闭，发送失败"),
}

// 接收：如果所有发送端都被 drop 了，且通道里没数据了，recv 会失败
match rx.recv() {
    Ok(value) => println!("收到: {}", value),
    Err(_) => println!("所有发送端都关闭了"),
}
```

`recv()` 是阻塞的——会一直等到有数据或通道关闭。

### 为什么必须 drop 原始 sender

接收循环靠"通道关闭"来判断结束。通道在**所有发送端都被 drop** 时才算关闭。

注意：worker 线程结束时，它们各自 `move` 进去的 `worker_tx` 副本会自动 drop。但主线程手里那个**原始的 `tx`** 如果不 drop，通道就永远不关闭，`rx` 的循环会一直卡着等数据：

```rust
// 关键一步：drop 原始 sender，否则接收循环永不结束
drop(tx);

for received in rx.iter() {  // 通道关闭后这个循环自然退出
    println!("收到: {}", received);
}
```

`rx.iter()` 返回一个迭代器，每一步都阻塞等待下一条消息，直到通道关闭。

### 消息传递 vs 共享可变状态

这道题的思路是：每个 worker 算出结果，**把结果发送回主线程**，由主线程统一汇总。这样就不需要 `Arc<Mutex<Vec<_>>>` 让多个线程争着写同一个数组。Rust 社区有句经验之谈——"不要通过共享内存来通信，而要通过通信来共享内存"。

---

## 本题覆盖

Practice message passing with `std::sync::mpsc`.

## 需要重点理解

- `mpsc::channel()` 返回 `(Sender, Receiver)`，发送端可 `clone`，接收端不可。
- `send` / `recv` 都返回 `Result`，不能 `unwrap`。
- 接收循环结束的前提是**所有发送端都被 drop**，所以要手动 `drop` 原始 sender。
- `rx.iter()` 会阻塞等待消息，通道关闭时自然结束。

## 常见坑

- 忘记 drop 原始 `tx`，接收循环一直阻塞，程序卡住不退出。
- 对 `send` / `recv` 用 `unwrap`，发送端或接收端关闭时会 panic。
- 用 `Arc<Mutex<Vec<_>>>` 让 worker 往同一个数组里写——违背了消息传递的初衷。
- 想把 `rx` 也 `clone` 了——`Receiver` 没有 `Clone`，编译都过不了。

## 回看问题

- 通道什么时候才算"关闭"？你的接收循环靠什么判断该停了？
- 如果一个 worker 在 send 之前就 panic 了，主线程会怎样？
- 这道题用消息传递，比用共享 `Mutex<Vec<_>>` 好在哪里？

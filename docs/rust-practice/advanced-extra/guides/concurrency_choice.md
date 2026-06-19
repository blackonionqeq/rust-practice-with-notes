# 多进程、多线程、async/await 怎么选

这篇是 advanced-extra 并发章节的拓展阅读。它不是新练习，而是帮助你在真实项目里判断：什么时候用多进程，什么时候用多线程，什么时候用 async/await。

## 三种执行模型

### 多进程

Rust 里通常用 `std::process::Command` 启动子进程。

适合：

- 调用外部命令，比如 `git`、`ffmpeg`、系统工具。
- 需要强隔离：子任务崩溃、内存泄漏或权限限制不应该拖垮主程序。
- 不同任务本来就是不同程序。

代价：

- 进程之间默认不共享内存。
- 通信通常靠 stdin/stdout、文件、socket 或其他 IPC，成本比线程高。

### 多线程

Rust 里可以直接用 `std::thread::spawn`，实际项目里做数据并行也常用 `rayon`。

适合：

- CPU 密集任务：压缩、解析、加密、图片处理、大量计算。
- 少量后台任务。
- 阻塞式 IO，且并发数量不大。
- 需要和同步代码、同步库直接配合。

代价：

- OS 线程比 async task 更重。
- 共享状态时要处理 `Arc`、`Mutex`、`RwLock`、channel、`Send`、`Sync`。
- 锁范围、死锁、panic 传播都要认真处理。

### async/await

Rust 的 `async fn` 会返回 `Future`。Future 需要 runtime 驱动，应用层最常见的是 Tokio。

适合：

- 大量 IO 并发：网络请求、服务器连接、数据库请求、定时器、异步文件 API。
- 很多任务大部分时间都在等待，而不是占着 CPU 计算。
- Web 服务、网络服务、爬虫、消息队列消费者等。

代价：

- 需要引入 runtime，比如 Tokio。
- async 代码会把函数边界染成 `async`，调用链要 `.await`。
- 不能在 async task 里随便做长时间 CPU 计算或阻塞调用，否则会占住 runtime worker 线程。

## 决策表

| 场景 | 优先选择 |
|---|---|
| 没有明确并发需求 | 普通同步代码 |
| CPU 密集计算 | 多线程，常用 `rayon` |
| 大量网络、数据库、定时器并发 | async/await，常用 Tokio |
| 少量后台任务，逻辑简单 | `std::thread` |
| 阻塞库必须调用，但程序主体是 async | `tokio::task::spawn_blocking` |
| 需要调用外部程序 | 多进程，`std::process::Command` |
| 需要强隔离、任务可能崩溃 | 多进程 |

## 简单决策树

```text
先问：真的需要并发吗？
    不需要 -> 普通同步代码
    需要 -> 继续

主要时间花在 CPU 计算？
    是 -> 多线程 / rayon
    否 -> 继续

主要时间花在等待 IO？
    是 -> async / Tokio
    否 -> 继续

需要调用外部程序或隔离崩溃？
    是 -> 多进程
    否 -> 线程通常更简单
```

## 为什么学习顺序先线程后 async

advanced-extra 里先安排线程，再安排 async，不是因为多线程永远比 async 更常用，而是因为线程能先训练 Rust 并发的底层约束：

- 所有权如何移动到另一个执行单元。
- 为什么线程闭包经常需要 `move`。
- `Send` / `Sync` 为什么重要。
- 什么时候用 channel，什么时候用共享状态。
- `Arc<Mutex<T>>`、`Arc<RwLock<T>>` 解决的是什么问题。
- 锁范围和死锁为什么是实际工程问题。

理解这些之后，再看 async task 会更清楚：async task 不是 OS 线程，但它同样会遇到所有权、生命周期、`Send`、共享状态、阻塞调用这些问题。

## 两个容易混淆的点

### async 不是更快的线程

async 的优势是高效等待大量 IO。它不是让 CPU 计算自动变快。

如果在 async task 里做很重的同步计算，会阻塞 runtime worker。Tokio 里遇到阻塞任务或 CPU 重活，通常要考虑：

```rust
tokio::task::spawn_blocking(|| {
    // 阻塞调用或 CPU-heavy work
});
```

如果是纯 CPU 并行计算，优先考虑线程池或 `rayon`。

### 并发不等于并行

并发表示多个任务在时间上交错推进；并行表示多个任务真的同时在多个 CPU 核上运行。

`tokio::join!` 可以并发等待多个 Future，但不保证它们都在不同 CPU 核上并行执行。多线程 runtime 下，spawn 出去的任务可能被调度到不同 worker 线程上；但 async 的主要价值仍然是让等待 IO 的任务把执行权让出来。

## 一句话总结

先写同步代码。CPU 并行用线程或 `rayon`，大量 IO 并发用 async/Tokio，需要隔离或外部命令用进程。

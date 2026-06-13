# 53 Async Fn and Await - 补充笔记

## 背景知识

### async fn 返回的是一个 Future，不是结果

普通函数调用立刻执行并返回结果。`async fn` 调用后返回的是一个**还没开始跑**的 `Future`，必须 `.await`（或 spawn）才会真正执行：

```rust
// 定义 async 函数
async fn fetch_label(id: u32) -> String {
    // ...
    format!("label-{}", id)
}

// 调用它：此时啥也没发生，只是拿到一个 Future
let future = fetch_label(42);   // 类型是 impl Future<Output = String>

// 必须 await 才会执行并拿到结果
let label = future.await;        // 现在才真正运行，返回 String
```

记住一句话：**async 函数的代码体在 `.await` 之前不会执行**。

### Future 需要一个 runtime 来驱动

Future 自己不会跑，需要一个"执行器"（runtime）去轮询它。Tokio 是最常用的 runtime。`#[tokio::main]` 把普通的 `fn main` 变成 runtime 入口：

```rust
#[tokio::main]
async fn main() {
    let label = fetch_label(42).await;
    println!("{}", label);
}
```

没有 runtime 直接调 `.await` 通常会报错——因为没人负责调度。

### 用 sleep 模拟异步等待

`tokio::time::sleep` 是异步的等待，它**不会阻塞当前线程**，而是让出执行权给其他任务：

```rust
use tokio::time::{sleep, Duration};

async fn fetch_label(id: u32) -> String {
    sleep(Duration::from_millis(50)).await;   // 异步等待，不阻塞线程
    format!("label-{}", id)
}
```

⚠️ 不要在 async 代码里用 `std::thread::sleep`——那是**阻塞**当前线程的，会把 runtime 的 worker 线程占住，整个 runtime 的并发能力都会受影响。

### 应用层 async，不用碰 Pin / 手写 Future

这一节只关心"怎么写 `async fn` 和 `.await`"。至于 `Future` 底层的轮询机制、`Pin` 为什么存在、怎么手写一个 `Future`——这些属于 runtime 实现细节，留到 expert 阶段。现在把它们当成"用就完了"的黑盒即可。

### Cargo.toml 里启用必要的 feature

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread", "time"] }
```

`macros` 提供 `#[tokio::main]`，`rt-multi-thread` 提供多线程 runtime，`time` 提供 `sleep`。

---

## 本题覆盖

Practice the surface syntax of `async fn` and `.await`.

## 需要重点理解

- `async fn` 调用返回 `Future`，必须 `.await` 或 spawn 才会执行。
- Future 需要 runtime（如 Tokio）驱动，`#[tokio::main]` 提供入口。
- `tokio::time::sleep` 是非阻塞等待；`std::thread::sleep` 在 async 里是禁忌。
- `Pin`、手写 `Future`、executor 原理不在本节范围。

## 常见坑

- 调用了 async 函数却忘了 `.await`，结果拿到的是 `Future` 而不是值。
- 在 async 任务里用 `std::thread::sleep` 阻塞了 runtime 的线程。
- 没有 runtime 就直接 `.await`，运行时报错。
- 把 async 的概念和 OS 线程混为一谈——async 任务是协作式调度的。

## 回看问题

- 你的 async 函数从被调用到拿到结果，中间发生了什么？什么时候才真正执行？
- 如果在 async 代码里需要一个延迟，应该用哪个 sleep？为什么？
- 这道题的 async 边界划在哪里——哪部分是异步操作，哪部分是普通计算？

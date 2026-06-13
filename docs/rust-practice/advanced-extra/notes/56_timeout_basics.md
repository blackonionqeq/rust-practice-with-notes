# 56 Timeout Basics - 补充笔记

## 背景知识

### tokio::time::timeout 给操作加期限

`tokio::time::timeout` 给一个 future 包一层"定时器"，超时就不再等它：

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_millis(100),
    some_async_operation(),
).await;
```

返回类型是 `Result<T, Elapsed>`：

- `Ok(value)` —— 在期限内完成，`value` 是原 future 的返回值。
- `Err(Elapsed)` —— 超时了，原 future 被"丢弃"（drop），不再继续。

注意这个嵌套的 `Result`：外层是"超时与否"，内层是"原操作自身的成功与否"。

### 区分两种错误

包了 timeout 之后，结果有两层 `Result`，要用两层匹配分清"超时"和"操作出错"：

```rust
match timeout(Duration::from_millis(100), fetch_data()).await {
    Ok(Ok(data))   => println!("按时拿到: {}", data),         // 没超时且成功
    Ok(Err(e))     => println!("按时完成但操作出错: {:?}", e),  // 没超时但失败
    Err(_elapsed)  => println!("超时了"),                       // 超时
}
```

把"超时"和"业务错误"混为一谈，是这道题最常见的坑。

### 用 sleep 模拟慢操作

和上一节一样，用 `tokio::time::sleep` 制造可控的延迟：

```rust
async fn slow_task() -> String {
    sleep(Duration::from_millis(300)).await;  // 模拟一个很慢的操作
    "done".to_string()
}
```

把 timeout 设成比 300ms 短（比如 100ms），就能稳定复现超时。

### 超时后会取消 future

timeout 超时时，原 future 会被 drop，相当于"取消"了这个操作。这意味着：

- 已经做了一半的工作会停止。
- 如果操作有副作用（比如已经发出去的网络请求），它不会因为 timeout 而"撤销"，只是你不再等它的结果了。

所以"取消"是一个需要谨慎对待的概念——本节只要理解到"timeout 会让 future 停止被轮询"即可，不必深究取消的安全性。

### 不要在 async 里用阻塞 sleep

再次强调：`std::thread::sleep` 会**阻塞当前线程**，在 async 代码里是禁忌。模拟延迟一律用 `tokio::time::sleep`：

```rust
// ✅ 对：异步 sleep
tokio::time::sleep(Duration::from_millis(100)).await;

// ❌ 错：阻塞整个 worker 线程
std::thread::sleep(Duration::from_millis(100));
```

---

## 本题覆盖

Practice applying timeouts to async operations.

## 需要重点理解

- `tokio::time::timeout` 返回 `Result<T, Elapsed>`，`Err` 表示超时。
- 包了 timeout 后结果是两层 `Result`：外层判超时，内层判操作本身的成败。
- 超时会让原 future 被 drop（取消），不再继续执行。
- 模拟延迟用 `tokio::time::sleep`，绝不用 `std::thread::sleep`。

## 常见坑

- 把超时和操作自身的错误混在一起，丢失了"到底是超时还是出错"的信息。
- 在 async 代码里用 `std::thread::sleep` 模拟慢操作，阻塞了 runtime。
- 对 timeout 的返回值 `unwrap`，超时直接 panic 而非优雅处理。
- 以为超时能"撤销"已发生的副作用——其实只是不再等结果。

## 回看问题

- 你的代码能否清楚地区分"操作成功"、"操作出错"、"操作超时"三种结果？
- 超时阈值设得合不合理？太短会误杀正常请求，太长形同虚设。
- 超时之后被取消的那个 future，有没有留下半成品状态需要清理？

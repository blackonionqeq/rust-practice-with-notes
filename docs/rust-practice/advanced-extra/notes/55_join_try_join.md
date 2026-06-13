# 55 Join and Try Join - 补充笔记

## 背景知识

### 顺序 await vs 并发 await

如果依次 `.await` 两个独立的异步操作，总耗时是两者之和（串行）：

```rust
// 串行：总时间 ≈ a 的时间 + b 的时间
let a = fetch_a().await;
let b = fetch_b().await;
```

如果它们互相独立，可以**并发**执行，总耗时约等于最慢那个的时间。Tokio 的 `join!` 宏就是干这个的：

```rust
use tokio::join;

// 并发：总时间 ≈ max(a 的时间, b 的时间)
let (a, b) = join!(fetch_a(), fetch_b());
```

`join!` 同时启动多个 future，等它们全部完成。

### join!：全部完成（不可失败的场景）

`join!` 适用于返回值不会失败、或你不在乎中途失败的 future。它会等**所有**操作跑完，再一起返回：

```rust
async fn fetch_label(id: u32) -> String { /* ... */ }

let (l1, l2) = join!(fetch_label(1), fetch_label(2));
```

### try_join!：任一失败就提前返回

如果操作返回 `Result`，你希望"只要有一个失败就立刻停止、不等其他"，用 `try_join!`：

```rust
use tokio::try_join;

async fn fetch_count(id: u32) -> Result<usize, AppError> { /* ... */ }

match try_join!(fetch_count(1), fetch_count(2)) {
    Ok((c1, c2)) => println!("总数: {}", c1 + c2),
    Err(e) => println!("出错了: {:?}", e),
}
```

`try_join!` 返回 `Result<(T1, T2), E>`：

- 全部成功 → `Ok((结果1, 结果2))`。
- 任一失败 → 立刻返回 `Err`，不等剩下的。

注意：要能用 `try_join!`，所有操作的**错误类型必须统一**（或能互相转换）。这道题就需要定义一个小的 `AppError`，让各 async 函数都返回 `Result<_, AppError>`。

### 什么时候用哪个

- 操作不会失败 / 都要结果 → `join!`，等全部完成。
- 操作会失败、且失败后没意义继续等 → `try_join!`，fail fast。
- 操作之间有依赖（B 需要 A 的结果）→ 老老实实顺序 `.await`。

> 注意 `join!` / `try_join!` 是**并发**，不一定是**并行**。在多线程 runtime（`rt-multi-thread`）下，独立的 future 可能被调度到不同线程真正并行执行。

---

## 本题覆盖

Practice waiting for multiple async operations.

## 需要重点理解

- 顺序 `.await` 是串行，总耗时是各操作之和；`join!` / `try_join!` 是并发，总耗时约等于最慢一个。
- `join!` 等所有操作完成；`try_join!` 遇到第一个错误就提前返回。
- 用 `try_join!` 要求各操作的错误类型统一（或可互相转换）。
- 有依赖关系的操作只能顺序 `.await`，独立操作才适合并发。

## 常见坑

- 本该并发却依次 `.await`，白白浪费时间。
- 用 `try_join!` 但各函数的错误类型不一致，编译报错。
- 操作之间有数据依赖却硬塞进 `join!`，结果拿不到前置数据。
- 对 `try_join!` 的返回值用 `unwrap`，吞掉了错误上下文。

## 回看问题

- 你的几个 async 操作是互相独立，还是有先后依赖？
- 这道题该用 `join!` 还是 `try_join!`？为什么？
- 你的 `AppError` 是否足够小、足够统一，能被所有 async 函数共用？

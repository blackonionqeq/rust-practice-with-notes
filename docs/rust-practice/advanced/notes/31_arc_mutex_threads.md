# 31 Arc Mutex Threads - 补充笔记

## 本题覆盖

- `Arc<T>`。
- `Mutex<T>`。
- 跨线程共享可变状态。
- `thread::spawn` 和 `join`。

## 需要补充

### `Arc<T>` 是线程安全的引用计数

`Rc<T>` 不能跨线程使用。跨线程共享所有权时，用：

```rust
Arc<T>
```

`Arc` 的计数操作是原子的，适合多线程。

### `Mutex<T>` 提供互斥可变访问

多个线程要修改同一个值时，需要同步：

```rust
Arc<Mutex<i32>>
```

`Arc` 负责共享所有权，`Mutex` 负责一次只允许一个线程修改。

### `lock` 返回 `Result`

```rust
let mut count = counter.lock().unwrap();
```

`lock` 可能失败，因为持有锁的线程 panic 会导致 mutex poisoned。

本练习可以用 `unwrap` 吗？题目没有禁止，但为了保持错误意识，项目里可以用 `expect("mutex poisoned")`。如果题目禁止 `unwrap/expect`，需要显式处理。

### 必须 join 线程

`thread::spawn` 返回 `JoinHandle`。

如果不 `join`，主线程可能先结束，或者无法确认子线程完成。

## 常见坑

- 在线程中直接移动同一个 `Arc`，导致后续循环不能再用。
- 忘记每个线程里 `Arc::clone`。
- 持锁范围过大。
- 忘记 `join`。

## 回看问题

- 为什么多线程用 `Arc` 而不是 `Rc`？
- `Mutex` 解决的是所有权问题，还是同步问题？
- 为什么要等待所有线程结束？

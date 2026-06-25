# 81 Safe Alternatives - 笔记

## 本题会用到的前置知识点

- `Vec`、`Option`、`Rc`。
- `Pin` 概念（79-80 题）。
- index 方案（77 题）。

## 核心结论

在需要"反向引用"或"自引用"时，绝大多数场景有更简单的安全替代方案，不需要 `Pin`。

## 三种方案对比

| 方案 | 适用场景 | 优点 | 缺点 |
|---|---|---|---|
| **Index** | 数据集中在 `Vec` 里 | 零额外开销，简单 | 需要手动维护 index 有效性 |
| **`Rc<T>`** | 需要多处共同持有所有权 | 安全，自动引用计数 | 堆分配，单线程，有循环引用风险 |
| **`Pin` + 裸指针** | 需要稳定内存地址（async runtime、自定义分配器） | 地址固定，可实现真正的自引用 | 复杂，需要 unsafe，维护 invariant 负担重 |

## 核心示例（index 方案）

```rust
struct Task { name: String, parent: Option<usize> }

fn print_chain(tasks: &[Task], mut idx: usize) {
    loop {
        print!("{}", tasks[idx].name);
        match tasks[idx].parent {
            Some(p) => { print!(" -> "); idx = p; }
            None => { println!(); break; }
        }
    }
}
```

## 核心示例（Rc 方案）

```rust
use std::rc::Rc;
struct TaskNode { name: String, parent: Option<Rc<TaskNode>> }

let root = Rc::new(TaskNode { name: "root".into(), parent: None });
let child = Rc::new(TaskNode { name: "child".into(), parent: Some(Rc::clone(&root)) });
println!("{}", child.parent.as_ref().unwrap().name); // root
```

## 何时才需要 Pin

- 实现异步 runtime 或自定义 Future。
- 手写 intrusive linked list（嵌入式/系统编程）。
- 需要把 Rust 数据固定给 C 侧持有指针的场景。

在学习阶段，遇到"需要自引用"时优先考虑 index 或 Rc。

## 常见坑

- 用 `Rc<RefCell<T>>` 建了循环引用导致内存泄漏——`Rc` 的引用计数无法处理环，需要 `Weak`。
- 以为 `Pin` 是解决所有自引用问题的通用方案——它是最后的手段，不是首选。

## 回看问题

- index 方案在什么情况下会出错？（data 被删除或重排后，老 index 失效）
- `Rc` 和 `Arc` 的区别是什么？本题为什么用 `Rc` 而不是 `Arc`？

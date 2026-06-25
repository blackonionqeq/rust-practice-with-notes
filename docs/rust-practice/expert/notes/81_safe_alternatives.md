# 81 Safe Alternatives - 笔记

## 本题会用到的前置知识点

- `Vec`、`Option`、`Rc`。
- `Pin` 概念（79-80 题）。
- index 方案（77 题）。

## 核心结论

在需要"反向引用"或"自引用"时，绝大多数场景有更简单的安全替代方案，不需要 `Pin`。

## 需要补充看懂的知识

### 先判断你要的是"关系"，还是"地址稳定"

很多所谓"自引用"需求，其实只是想表达对象之间的关系：任务有父任务、节点有上级、记录指向另一条记录。这类需求通常用 index、id、`Rc`、`Arc` 就够了。

`Pin` 解决的是更底层的问题：某个值内部或外部保存了指向它内存地址的指针，所以这个值一旦移动，指针就会失效。只有真的依赖地址稳定时，才需要考虑 `Pin`。

### index/id 方案的关键是维护有效性

index 方案简单高效，但它把正确性转移到容器管理上。只要 `Vec` 删除、交换、排序元素，旧 index 就可能指向错误任务。

更稳妥的变体是使用稳定 id，例如 `TaskId(usize)` 加 `HashMap<TaskId, Task>`，或者只允许追加、不允许重排的 arena。学习阶段先用 index，重点是理解"不保存引用"这个思路。

### `Rc` 解决共享所有权，不解决可变图的一切问题

`Rc<T>` 适合多个节点共享读取同一个父节点。它不允许直接可变借用内部数据；需要可变图时常会搭配 `RefCell<T>`，但这会把借用检查推迟到运行时。

如果父子互相持有 `Rc`，引用计数会形成环，导致内存泄漏。真实双向关系通常要让其中一边用 `Weak<T>`，不过本题暂时不展开。

### 选择顺序

学习和日常业务代码里可以按这个顺序考虑：

1. 只需要表达关系：先用 index/id。
2. 需要共享读取所有权：用 `Rc<T>`（多线程用 `Arc<T>`）。
3. 真的需要稳定地址、并愿意维护 unsafe invariant：再考虑 `Pin`。

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

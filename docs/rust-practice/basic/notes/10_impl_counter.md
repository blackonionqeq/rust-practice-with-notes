# 10 Impl Counter - 补充笔记

## 本题覆盖

- `impl` 方法块。
- 关联函数 `Counter::new()`。
- `self`、`&self`、`&mut self`。
- 封装字段访问。

## 需要补充

### 方法和关联函数

写在 `impl Counter` 里的函数分两类：

- 第一个参数是 `self`、`&self`、`&mut self`：方法，用 `counter.current()` 调用。
- 没有 `self` 参数：关联函数，用 `Counter::new()` 调用。

`new` 不是语言关键字，只是 Rust 社区常用的构造函数命名。

### `&self` 表示只读借用

```rust
fn current(&self) -> i32
```

这个方法只读取 `value`，不取得整个结构体所有权，也不修改它。

### `&mut self` 表示可变借用

```rust
fn increment(&mut self)
```

调用这个方法时，变量本身必须是可变绑定：

```rust
let mut counter = Counter::new();
counter.increment();
```

### `self` 表示消费当前值

本题没有使用 `self`。如果方法签名是：

```rust
fn into_value(self) -> i32
```

调用后整个 `Counter` 会被移动，原变量不能继续使用。

### 通过方法隐藏字段

本题要求 `main` 不直接访问 `counter.value`，是为了练习通过方法暴露行为：

- 外部只知道可以 `increment`、`add`、`reset`。
- 字段未来改名或改变内部实现时，外部代码受影响更小。

## 常见坑

- 修改字段的方法忘记写 `&mut self`。
- 创建变量时忘记 `let mut counter`。
- 把 `Counter::new()` 写成 `counter.new()`。

## 回看问题

- 方法和关联函数的调用方式有什么区别？
- `self`、`&self`、`&mut self` 分别表示什么？
- 为什么通过方法访问字段更利于维护？

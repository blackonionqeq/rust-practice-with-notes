# 33 CLI Args and PathBuf - 补充笔记

## 背景知识

### 获取命令行参数

```rust
use std::env;

let args: Vec<String> = env::args().collect();
```

`env::args()` 返回一个迭代器，第 0 个元素永远是程序本身的名称（如 `./cli_args_pathbuf`），第 1 个才是用户传入的第一个参数。所以取路径要用 `args.get(1)`，而不是 `args.get(0)`。

通常更惯用的写法是跳过第一个，直接拿第二个：

```rust
let path_arg = env::args().nth(1); // Option<String>
```

### PathBuf 与 &Path

`PathBuf` 是拥有所有权的路径（类似 `String`），`&Path` 是借用的路径切片（类似 `&str`）。  
从 `String` 或字面量构造：

```rust
use std::path::PathBuf;

let p = PathBuf::from("sample.txt");
```

需要传给接受 `&Path` 的函数时，直接传 `&p` 即可，Rust 会自动 deref。

### 为什么用 .display() 打印路径

`PathBuf` / `Path` 不直接实现 `Display`，因为路径在不同平台上编码不同（Windows 可能是 UTF-16）。`.display()` 返回一个可以安全打印的包装类型：

```rust
println!("{}", p.display()); // 正确
// println!("{}", p);        // 编译错误
```

### 读取文件

```rust
use std::fs;

let content = fs::read_to_string(&p); // 返回 Result<String, std::io::Error>
```

不要 `unwrap`，用 `match` 或 `?` 处理错误，并给用户一条清晰的错误提示。

### 如何传参数运行

```bash
cargo run -- sample.txt
```

`--` 之后的内容会传给你的程序，而不是 Cargo 本身。

### stdin 与 CLI 参数的区别

- **CLI 参数**（本题用法）：程序启动时由 shell 传入，通过 `env::args()` 读取，适合传文件路径、选项等配置信息。
- **stdin**：程序运行中通过管道或键盘输入，通过 `std::io::stdin().read_line(&mut buf)` 读取，适合交互式输入或流式数据。

---

## 本题覆盖

Practice reading command line arguments and using `PathBuf` for file paths.

## 需要重点理解

- 真实 CLI 程序不能总是假设固定文件名。
- `PathBuf` / `Path` 比 `String` 更适合表达文件系统路径。
- 大文件处理优先考虑 streaming IO。
- 配置解析应该和业务逻辑分开。

## 常见坑

- 为了图方便使用 `unwrap`，导致错误信息不清楚。
- 把 CLI、IO、业务逻辑全部写在一个函数里，后续难以测试。
- 过早引入复杂抽象，而不是先保持清晰边界。

## 回看问题

- 这个题目里的边界在哪里：CLI、IO、解析、业务逻辑还是并发/异步？
- 哪些错误应该交给调用者处理？哪些可以在 binary 层转换成用户提示？
- 当前设计是否容易写测试？

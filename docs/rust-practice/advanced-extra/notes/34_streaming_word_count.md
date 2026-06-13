# 34 Streaming Word Count - 补充笔记

## 背景知识

### 为什么不用 read_to_string

`fs::read_to_string` 会把文件全部加载进内存。对 GB 级文件这会导致内存爆炸。  
`BufReader` 在内部维护一个固定大小的缓冲区，每次只读一小块，适合处理任意大文件。

### File::open

```rust
use std::fs::File;

let file = File::open(path)?; // 返回 Result<File, io::Error>
```

`File` 本身只是一个文件句柄，不带缓冲。需要套上 `BufReader` 才能按行高效读取。

### BufReader 与 BufRead trait

```rust
use std::io::{BufRead, BufReader};

let reader = BufReader::new(file);
```

**注意**：`.lines()` 方法来自 `BufRead` trait，必须把这个 trait `use` 进作用域才能调用。

### .lines() 迭代器

`.lines()` 返回 `impl Iterator<Item = io::Result<String>>`——每一行都是一个 `Result`，因为读取过程中随时可能发生 IO 错误。

```rust
for line in reader.lines() {
    let line = line?;          // 传播 IO 错误
    // line 现在是 String
}
```

### 统计单词数

`str::split_whitespace()` 按任意空白（空格、Tab、换行）分词，比 `.split(' ')` 更健壮：

```rust
let count = line.split_whitespace().count();
```

### 函数签名参考

```rust
fn count_words_in_file(path: &std::path::Path) -> std::io::Result<usize> {
    // ...
}
```

接受 `&Path` 而非 `&PathBuf`，这样调用方传 `&PathBuf`、`&str`、`Path` 都能兼容。

---

## 本题覆盖

Practice processing a file line by line with `BufReader` instead of loading the whole file.

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

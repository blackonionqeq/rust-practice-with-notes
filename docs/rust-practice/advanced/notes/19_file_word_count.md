# 19 File Word Count - 补充笔记

## 本题覆盖

- 文件读取。
- `std::fs::read_to_string`。
- `std::io::Error`。
- `?` 传播 IO 错误。
- 文本按空白分词。

## 需要补充

### `read_to_string` 读取整个文件

```rust
std::fs::read_to_string(path)
```

返回：

```rust
Result<String, std::io::Error>
```

成功时得到文件内容，失败时得到 IO 错误，例如文件不存在、权限不足、编码不合法等。

### 路径是相对当前工作目录

`run("sample.txt")` 会从程序运行时的当前目录找文件。

用 `cargo run` 时，当前目录通常是项目根目录，所以 `sample.txt` 应放在 `Cargo.toml` 旁边。

### `split_whitespace` 适合基础分词

```rust
text.split_whitespace().count()
```

它会按连续空白分割，包括空格、换行、制表符。

这不是完整自然语言分词，但足够做基础文本统计。

### IO 错误不要 `unwrap`

文件不存在是正常运行时情况，不应该直接 panic。

用 `Result` 让调用方决定如何处理：

```rust
fn run(path: &str) -> Result<usize, std::io::Error> {
    let text = read_text(path)?;
    Ok(count_words(&text))
}
```

## 常见坑

- `sample.txt` 放到 `src/` 下，导致运行时找不到。
- 忘记 `run` 返回 `Result`，却在里面使用 `?`。
- 使用 `.lines().count()` 当作单词数。
- 用 `unwrap` 处理文件读取。

## 回看问题

- `read_to_string` 的错误类型是什么？
- 相对路径是相对源文件，还是相对运行时工作目录？
- `split_whitespace` 和 `lines` 的用途有什么区别？

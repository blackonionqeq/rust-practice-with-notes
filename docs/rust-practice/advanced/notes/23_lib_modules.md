# 23 Lib Modules - 补充笔记

## 本题覆盖

- `src/lib.rs`。
- binary crate 调用 library crate。
- 多文件模块。
- `crate::` 路径。

## 需要补充

### 一个 package 可以同时有 library 和 binary

当项目里有：

```text
src/lib.rs
src/main.rs
```

Cargo 会构建：

- 一个 library crate。
- 一个 binary crate。

`main.rs` 可以通过 package name 使用 library crate：

```rust
lib_modules::stats::summarize(text)
```

### `lib.rs` 是库 crate 根

在 `src/lib.rs` 中：

```rust
pub mod stats;
pub mod format;
```

表示声明并导出模块。默认会寻找：

```text
src/stats.rs
src/format.rs
```

### `crate::` 指当前 crate 根

在 library 内部：

```rust
crate::stats::Summary
```

这里的 `crate` 指 `src/lib.rs` 代表的库 crate 根。

在 `main.rs` 里，`crate` 指 binary crate 根，不是 library crate 根。

### 模块边界应服务可读性

把统计逻辑放 `stats.rs`，格式化逻辑放 `format.rs`，是为了让每个文件职责单一。

如果为了拆而拆，反而会增加路径和可见性负担。

## 常见坑

- 在 `main.rs` 里写 `crate::stats::summarize`，但 `stats` 在 library crate 中。
- 忘记在 `lib.rs` 里 `pub mod stats;`。
- 模块文件存在，但没有被 `mod` 声明。
- 结构体是 `pub`，字段没有 `pub`，外部仍然访问不到字段。

## 回看问题

- package、library crate、binary crate 的关系是什么？
- `crate::` 在 `lib.rs` 和 `main.rs` 中分别指什么？
- 什么时候应该拆模块？

# 36 Clap Text Stats - 补充笔记

## 本题覆盖

Practice using the `clap` crate for a small typed CLI.

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

# 45 Workspace Basics - 补充笔记

## 本题覆盖

Practice organizing multiple crates in one Cargo workspace.

## 需要重点理解

- integration tests 应该从使用者视角测试 public API。
- doc tests 既是文档也是回归测试。
- workspace 用来组织多个相关 crate。
- feature flags 应该让默认构建保持轻量。

## 常见坑

- 为了图方便使用 `unwrap`，导致错误信息不清楚。
- 把 CLI、IO、业务逻辑全部写在一个函数里，后续难以测试。
- 过早引入复杂抽象，而不是先保持清晰边界。

## 回看问题

- 这个题目里的边界在哪里：CLI、IO、解析、业务逻辑还是并发/异步？
- 哪些错误应该交给调用者处理？哪些可以在 binary 层转换成用户提示？
- 当前设计是否容易写测试？

# 53 Async Fn and Await - 补充笔记

## 本题覆盖

Practice the surface syntax of `async fn` and `.await`.

## 需要重点理解

- advanced-extra 的 async 只关注应用层写法。
- `async fn` 返回 future，实际执行需要 runtime、`.await` 或 spawn。
- 不要在 async 任务里做阻塞等待。
- `Pin`、手写 Future 和 executor 原理留到 expert 阶段。

## 常见坑

- 为了图方便使用 `unwrap`，导致错误信息不清楚。
- 把 CLI、IO、业务逻辑全部写在一个函数里，后续难以测试。
- 过早引入复杂抽象，而不是先保持清晰边界。

## 回看问题

- 这个题目里的边界在哪里：CLI、IO、解析、业务逻辑还是并发/异步？
- 哪些错误应该交给调用者处理？哪些可以在 binary 层转换成用户提示？
- 当前设计是否容易写测试？

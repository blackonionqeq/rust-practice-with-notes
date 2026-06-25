# 64 Macro Repetition Join - 笔记

## 本题会用到的前置知识点

- `String` 基础。
- 迭代与拼接思路。
- 宏基础定义与调用。

## 核心结论

- `$(...),+` 用于“一个或多个”重复参数匹配。
- 重复模式适合处理可变参数数量。
- 入门先做线性、可读的展开逻辑。

## 核心示例

```rust
macro_rules! join_words {
    ($first:expr $(, $rest:expr)+) => {{
        let mut out = String::from($first);
        $(
            out.push('-');
            out.push_str($rest);
        )+
        out
    }};
    ($single:expr) => {
        String::from($single)
    };
}
```

理解重点：重复模式常要考虑“单个参数”和“多个参数”两种分支。

## 常见坑

- 只写多参数分支，漏掉单参数情况。
- 展开逻辑过度花哨，调试困难。

## 回看问题

- 宏是否正确覆盖 1 个与多个输入？
- 能否用更直观的实现保持可读性？
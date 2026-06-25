# 64 Macro Repetition Join - 笔记

## 本题会用到的前置知识点

- `String` 基础。
- 迭代与拼接思路。
- 宏基础定义与调用。

## 核心结论

- `$(...),+` 用于“一个或多个”重复参数匹配。
- 重复模式适合处理可变参数数量。
- 重复 matcher 里的分隔符要写清楚：`$(, $rest:expr)+` 表示每次重复前都有一个逗号。
- `+` 表示至少一次，`*` 表示零次或多次。
- 入门先做线性、可读的展开逻辑。

## 需要补充看懂的知识

### 重复 matcher 的读法

```rust
($first:expr $(, $rest:expr)+)
```

可以拆开读：

- `$first:expr`：先匹配第一个表达式。
- `$( ... )+`：括号里的模式重复一次或多次。
- `, $rest:expr`：每次重复都匹配“一个逗号 + 一个表达式”。

所以它能匹配：

```rust
join_words!("a", "b")
join_words!("a", "b", "c")
```

但不能单独匹配：

```rust
join_words!("a")
```

因此示例里还需要单参数分支。

### `+`、`*`、`,` 的位置很重要

常见形式：

```rust
$($item:expr),+
```

表示“一个或多个表达式，中间用逗号分隔”。它能匹配：

```rust
"a"
"a", "b", "c"
```

而本题使用：

```rust
$first:expr $(, $rest:expr)+
```

是为了把第一个元素单独拿出来，避免拼接字符串时处理开头多余的 `-`。

### 宏展开里的重复也要对应

匹配处用了重复变量，展开处通常也要放进重复块：

```rust
$(
	let part = $rest;
	out.push('-');
	out.push_str(part.as_ref());
)+
```

这里的意思是：每匹配到一个 `$rest`，就展开一份这段代码。

如果调用：

```rust
join_words!("macro", "rules", "ok")
```

展开后可以粗略理解为：

```rust
let mut out = String::from("macro");
let part = "rules";
out.push('-');
out.push_str(part.as_ref());
let part = "ok";
out.push('-');
out.push_str(part.as_ref());
out
```

### 为什么用两层花括号

```rust
($first:expr $(, $rest:expr)+) => {{
	let mut out = String::from($first);
	// ...
	out
}};
```

外层 `{ ... }` 是 macro arm 的展开体；内层 `{ ... }` 是 Rust 块表达式。这个块可以创建局部变量 `out`，最后一行返回 `out`。

## 核心示例

```rust
macro_rules! join_words {
    ($first:expr $(, $rest:expr)+) => {{
        let mut out = String::from($first);
        $(
            let part = $rest;
            out.push('-');
            out.push_str(part.as_ref());
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
- 把 `+` 和 `*` 混用，导致是否允许空输入不清楚。
- 分隔符位置写错，导致匹配不到预期调用形式。
- 匹配处用了重复变量，展开处忘记也放进重复结构。
- 展开逻辑过度花哨，调试困难。

## 回看问题

- 宏是否正确覆盖 1 个与多个输入？
- 能否用更直观的实现保持可读性？
- 重复 matcher 是否能用普通话读清楚？

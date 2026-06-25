# 65 Macro Hygiene Basics - 笔记

## 本题会用到的前置知识点

- 局部变量作用域。
- 表达式求值。
- 宏基础匹配与展开。

## 核心结论

- 宏内部临时变量应尽量避免干扰调用方命名。
- 宏参数表达式要注意“只求值一次”。
- 读代码时要先看展开后是否清晰。

## 核心示例

```rust
macro_rules! measure_len {
    ($input:expr) => {{
        let __value = $input;
        __value.to_string().len()
    }};
}

fn main() {
    let tmp = 10;
    let n = measure_len!("rust");
    println!("{tmp}, {n}");
}
```

理解重点：宏内部变量名与调用方同名变量不应造成语义混乱。

## 常见坑

- 在宏里多次使用 `$input`，导致重复求值。
- 宏体隐式依赖调用方环境。

## 回看问题

- 输入表达式是否只执行了一次？
- 展开后代码在作用域上是否清楚？
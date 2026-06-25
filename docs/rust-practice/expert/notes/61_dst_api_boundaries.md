# 61 DST-Friendly API Boundaries - 笔记

## 本题会用到的前置知识点

- `&str`、`&[T]` 切片。
- 所有权与借用。
- 迭代器基础。

## 核心结论

- 只读文本优先用 `&str`，只读序列优先用 `&[T]`。
- 这类边界天然兼容 owned 与 borrowed 输入。
- 学习阶段优先“清晰签名”，不要过早抽象。

## 核心示例

```rust
fn first_word(input: &str) -> &str {
	input.split_ascii_whitespace().next().unwrap_or("")
}

fn sum_slice(values: &[i32]) -> i32 {
	values.iter().sum()
}

fn main() {
	let s = String::from("rust lang");
	let arr = [1, 2, 3];
	let vec = vec![4, 5];
	println!("{}", first_word(&s));    // rust
	println!("{}", first_word("go now")); // go
	println!("{}", sum_slice(&arr));   // 6
	println!("{}", sum_slice(&vec));   // 9
}
```

理解重点：相同签名可以覆盖多种容器，不需要把所有权拿进函数。

## 常见坑

- 只读场景却把参数写成 `String` / `Vec<T>`。
- 为了“通用”引入不必要的复杂约束。

## 回看问题

- 函数是否不必要地拿走了所有权？
- 当前签名对 `String`、字面量、切片是否都友好？

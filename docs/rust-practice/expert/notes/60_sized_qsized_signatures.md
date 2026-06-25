# 60 Sized and ?Sized Signatures - 笔记

## 本题会用到的前置知识点

- 泛型与 trait bound。
- 引用和切片。
- `String`、`Vec<u8>` 的基础使用。

## 核心结论

- 泛型默认带 `Sized` 约束。
- `?Sized` 允许接收动态大小类型，但通常要通过引用/指针。
- 本题重点是“签名设计”，不是复杂泛型技巧。

## 核心示例

```rust
fn byte_len<T: ?Sized + AsRef<[u8]>>(value: &T) -> usize {
	value.as_ref().len()
}

fn main() {
	let s = String::from("abc");
	let v = vec![1_u8, 2, 3, 4];
	println!("{}", byte_len(&s));       // 3
	println!("{}", byte_len("hello")); // 5
	println!("{}", byte_len(&v));       // 4
}
```

理解重点：`?Sized` 配合 `&T` 使用，才能自然覆盖更多输入形态。

## 常见坑

- 写了 `T: ?Sized` 却仍按值接收 `T`。
- 为了通用性过度抽象，代码可读性下降。

## 回看问题

- 这里真的需要泛型吗？
- 用 `&str` / `&[T]` 能否更直接表达需求？

# 58 Static Lifetime Basics - 笔记

## 本题会用到的前置知识点

- 引用与生命周期标注。
- 所有权与借用。
- 泛型函数与 trait bound 基础。

## 核心结论

- `&'static T`：约束“这个引用本身要活得足够久”。
- `T: 'static`：约束“这个类型不依赖外部短生命周期借用”。
- 二者关注点不同，不要混用。

## 核心示例

```rust
fn takes_static_ref(s: &'static str) -> usize { s.len() }
fn takes_static_bound<T: 'static>(v: T) -> T { v }

fn main() {
	let a = takes_static_ref("rust"); // 字面量通常可作为 &'static str
	let b = takes_static_bound(String::from("ok")); // String 常满足 T: 'static
	println!("{a}, {b}");
}
```

理解重点：上面第二行成立，不代表 `&String` 自动就是 `&'static String`。

## 常见坑

- 把 `&'static str` 和 `String` 视为同一种约束。
- 看到 `'static` 就误以为必须用全局变量。

## 回看问题

- 当前 `'static` 是作用在“引用”还是“类型 bound”？
- 我需要的是“长生命周期引用”还是“可独立持有的类型”？

# 59 Box Leak (Intentional) - 笔记

## 本题会用到的前置知识点

- `Box<T>` 与堆分配基础。
- `String` 与 `&str` 转换关系。
- 所有权移动。

## 核心结论

- `Box::leak` 会返回长期可用引用（常见为 `&'static str`）。
- 这是“有意不释放”内存，不是默认做法。
- 仅在数据量可控、生命周期与进程一致时考虑。

## 核心示例

```rust
fn make_static_label(input: String) -> &'static str {
	Box::leak(input.into_boxed_str())
}

fn main() {
	let label = make_static_label("boot-once".to_string());
	println!("{label}");
}
```

理解重点：这个 `label` 可长期使用，但对应内存在进程结束前不会释放。

## 常见坑

- 把 leak 当常规手段使用。
- 在循环里不断 leak，造成持续增长。
- 不写明为什么这个泄漏是可接受的。

## 回看问题

- 这块内存规模是否可控？
- 能否用普通所有权模型替代？

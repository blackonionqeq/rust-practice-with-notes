# 59 Box Leak (Intentional) - 笔记

## 本题会用到的前置知识点

- `Box<T>` 与堆分配基础。
- `String` 与 `&str` 转换关系。
- 所有权移动。

## 核心结论

- `Box::leak` 会返回长期可用引用（常见为 `&'static str`）。
- `Box::leak` 会消耗 `Box<T>`，并放弃自动释放这块堆内存。
- 这是“有意不释放”内存，不是默认做法。
- 仅在数据量可控、生命周期与进程一致时考虑。

## 需要补充看懂的知识

### 从 `String` 到 `&'static str` 的链路

本题代码通常分两步：

```rust
fn make_static_label(input: String) -> &'static str {
	Box::leak(input.into_boxed_str())
}
```

拆开看：

```rust
let boxed: Box<str> = input.into_boxed_str();
let leaked: &'static mut str = Box::leak(boxed);
let shared: &'static str = leaked;
```

含义是：

- `input` 原来拥有字符串内容。
- `into_boxed_str()` 把字符串内容变成堆上的 `Box<str>`。
- `Box::leak()` 消耗这个 box，不再让 Rust 在作用域结束时释放它。
- 因为这块内存不再自动释放，返回的引用可以被当成 `'static`。

### leak 是一条单向路

普通 `String` 离开作用域会释放内存：

```rust
{
	let s = String::from("temp");
} // s 在这里释放
```

leak 后不同：

```rust
let label: &'static str = make_static_label(String::from("boot"));
```

`label` 指向的堆内存在进程结束前不会被释放。你不再持有原来的 `Box<str>`，也就没有常规方式把它交还给 Rust 自动管理。

### 为什么不用 `&input`

下面这种思路不成立：

```rust
// 错误思路：input 是函数参数，函数结束就会释放
// fn make_static_label(input: String) -> &'static str {
// 	&input
// }
```

`input` 在函数返回时会被释放，返回指向它的引用会悬垂。`Box::leak` 的作用正是改变所有权结局：让数据不再被释放，从而可以返回长期引用。

### 什么时候能接受 leak

可以考虑的场景：

- 程序启动时加载少量配置、标签、注册表项。
- 数据数量固定或有明确上限。
- 数据生命周期本来就等同于整个进程。

不适合的场景：

- 每次请求、每轮循环、每个用户输入都 leak。
- 数据规模不可控。
- 只是为了绕过生命周期错误。

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
- 以为 `Box::leak` 只是“延长生命周期”，忘记它同时放弃释放。
- 用 leak 掩盖本该通过所有权设计解决的问题。

## 回看问题

- 这块内存规模是否可控？
- 能否用普通所有权模型替代？
- 这份数据是否真的应该活到进程结束？

# 60 Sized and ?Sized Signatures - 笔记

## 本题会用到的前置知识点

- 泛型与 trait bound。
- 引用和切片。
- `String`、`Vec<u8>` 的基础使用。

## 核心结论

- 泛型默认带 `Sized` 约束。
- `?Sized` 允许接收动态大小类型，但通常要通过引用/指针。
- `str`、`[T]`、`dyn Trait` 这类 DST 不能直接按值传递，因为编译器不知道值本体占多少字节。
- `&str`、`&[T]`、`Box<dyn Trait>` 这类指针本身大小固定，所以可以作为函数参数、局部变量、集合元素使用。
- 本题重点是“签名设计”，不是复杂泛型技巧。

## 需要补充看懂的知识

### `Sized` 是“编译期知道大小”

Rust 默认要求普通局部变量、函数按值参数、结构体字段、泛型参数的大小在编译期确定。

```rust
fn take_value<T>(value: T) {
	let _ = value;
}
```

上面的 `T` 实际上等价于：

```rust
fn take_value<T: Sized>(value: T) {
	let _ = value;
}
```

这就是为什么大多数泛型函数不用写 `T: Sized`：它是默认限制。

### DST 是“值本体大小不固定”

DST 是 dynamically sized type，动态大小类型。常见例子：

- `str`：文本长度不固定。
- `[u8]` / `[T]`：切片元素个数不固定。
- `dyn Trait`：具体实现类型可能不同。

注意区别：

```rust
let s: &str = "hello";
let bytes: &[u8] = &[1, 2, 3];
```

这里的变量不是裸 `str` 或裸 `[u8]`，而是 `&str` 和 `&[u8]`。引用本身大小固定，引用指向的数据长度可以变化。

### 为什么 DST 要放在引用或指针后面

函数调用、栈上局部变量、`Vec<T>` 元素布局都需要知道每个值占多少字节。裸 `str` 或裸 `[u8]` 没有固定大小，所以不能这样写：

```rust
// 错误思路：str 本体大小未知，不能按值接收
// fn bad(input: str) -> usize {
// 	input.len()
// }
```

但下面可以：

```rust
fn ok(input: &str) -> usize {
	input.len()
}
```

`&str` 是一个胖指针：它包含数据地址和长度。`&[T]` 也是类似结构：数据地址和元素个数。指针本身大小固定，所以可以传来传去。

### `?Sized` 是“放宽默认限制”

`T: ?Sized` 不是说 `T` 必须是动态大小类型，而是说：`T` 可以是 `Sized`，也可以不是 `Sized`。

```rust
fn byte_len<T: ?Sized + AsRef<[u8]>>(value: &T) -> usize {
	value.as_ref().len()
}
```

这个签名里有两个重点：

- `T: ?Sized`：取消泛型参数默认的 `Sized` 限制。
- `value: &T`：即使 `T` 可能大小未知，也只通过引用接收它。

如果写成下面这样，仍然不成立：

```rust
// 错误思路：T 可能大小未知，却试图按值接收
// fn byte_len_bad<T: ?Sized + AsRef<[u8]>>(value: T) -> usize {
// 	value.as_ref().len()
// }
```

### `T` 在不同调用里可能不同

同一个函数签名可以覆盖这些输入：

```rust
let s = String::from("abc");
let v = vec![1_u8, 2, 3, 4];
let slice: &[u8] = &[9, 8, 7];

println!("{}", byte_len(&s));      // T = String
println!("{}", byte_len("hello")); // T = str
println!("{}", byte_len(&v));      // T = Vec<u8>
println!("{}", byte_len(slice));   // T = [u8]
```

这里最容易卡住的是 `"hello"` 和 `slice`：

- `"hello"` 的类型是 `&'static str`，传给 `&T` 时，`T` 可以是 `str`。
- `slice` 的类型是 `&[u8]`，传给 `&T` 时，`T` 可以是 `[u8]`。

所以本题真正练的是：当你想让泛型函数既接受普通有大小类型，也接受 DST 时，参数要写成 `&T`，并给 `T` 加 `?Sized`。

## 核心示例

```rust
fn byte_len<T: ?Sized + AsRef<[u8]>>(value: &T) -> usize {
	value.as_ref().len()
}

fn main() {
	let s = String::from("abc");
	let v = vec![1_u8, 2, 3, 4];
	let slice: &[u8] = &[9, 8, 7];
	println!("{}", byte_len(&s));       // 3
	println!("{}", byte_len("hello")); // 5
	println!("{}", byte_len(&v));       // 4
	println!("{}", byte_len(slice));    // 3
}
```

理解重点：`?Sized` 配合 `&T` 使用，才能自然覆盖更多输入形态。

## 和第 61 题的边界

第 60 题让你看懂 `?Sized` 签名为什么能工作。第 61 题会回到更常用的 API 设计：只读字符串优先写 `&str`，只读字节或数组优先写 `&[T]`。

实际写业务代码时，不要为了“高级”就到处写 `T: ?Sized + AsRef<_>`。如果函数只需要文本，`fn f(input: &str)` 往往更清楚；如果函数只需要一段元素，`fn f(values: &[T])` 往往更直接。

## 常见坑

- 写了 `T: ?Sized` 却仍按值接收 `T`。
- 把 `?Sized` 理解成“这个类型一定没有固定大小”。它只是允许没有固定大小。
- 忘记裸 `str` / 裸 `[T]` 和 `&str` / `&[T]` 不是一回事。
- 为了通用性过度抽象，代码可读性下降。

## 回看问题

- 这里真的需要泛型吗？
- 用 `&str` / `&[T]` 能否更直接表达需求？
- 这个函数是否只通过引用或智能指针接收可能的 DST？

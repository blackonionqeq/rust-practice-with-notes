# 58 Static Lifetime Basics - 笔记

## 本题会用到的前置知识点

- 引用与生命周期标注。
- 所有权与借用。
- 泛型函数与 trait bound 基础。

## 核心结论

- `&'static T`：约束“这个引用本身要活得足够久”。
- `T: 'static`：约束“这个类型不依赖外部短生命周期借用”。
- 字符串字面量常见为 `&'static str`，但普通局部变量的引用通常不是 `'static`。
- `String` 这类拥有数据的类型通常满足 `T: 'static`，因为它不借用外部局部数据。
- 二者关注点不同，不要混用。

## 需要补充看懂的知识

### `'static` 不是“变量永远存在”

`'static` 表示“这个引用可以被当作活到程序结束”，或“这个类型不包含短生命周期借用”。它不是说所有相关变量都真的永远不销毁。

```rust
let s = String::from("owned");
```

这个 `s` 仍然会在作用域结束时被释放。但 `String` 本身拥有堆上的文本，不借用某个局部变量，所以把它作为 `T` 传给 `T: 'static` 的泛型函数通常没问题。

### `&'static T` 约束的是引用

```rust
fn takes_static_ref(s: &'static str) -> usize {
	s.len()
}
```

这个函数要求传入的引用本身足够长寿。字符串字面量可以：

```rust
let n = takes_static_ref("rust");
```

但下面这种思路不行：

```rust
// 错误思路：local 是局部 String，&local 不能活到程序结束
// let local = String::from("rust");
// takes_static_ref(&local);
```

`local` 离开当前作用域就会释放，指向它内容的引用不能被当成 `&'static str`。

### `T: 'static` 约束的是类型里不要藏短借用

```rust
fn takes_static_bound<T: 'static>(value: T) -> T {
	value
}
```

`T: 'static` 并不要求传进来的值永远不释放。它要求 `T` 这个类型不依赖某个较短生命周期的引用。

```rust
let owned = String::from("ok");
let owned = takes_static_bound(owned);
```

这可以通过，因为 `String` 拥有数据。

反过来，如果类型里包含局部借用，就不满足：

```rust
// 错误思路：&local 借用了局部变量 local
// let local = String::from("short");
// takes_static_bound(&local);
```

这里 `T` 会类似于 `&String`，而这个引用的生命周期跟 `local` 绑定，不是 `'static`。

### 两个问题分开问

看到 `'static` 时先判断它写在哪里：

```rust
&'static str      // 引用必须足够长寿
T: 'static       // T 不能依赖短借用
Box<dyn Error + 'static> // trait object 里不能装短借用错误
```

本题只要求分清前两个。后面的 trait object 形式以后遇到错误处理和 trait object 时再展开。

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
- 以为 `T: 'static` 会让值不释放。
- 忘记先判断 `'static` 是标在引用上，还是标在泛型 bound 上。

## 回看问题

- 当前 `'static` 是作用在“引用”还是“类型 bound”？
- 我需要的是“长生命周期引用”还是“可独立持有的类型”？

# 61 DST-Friendly API Boundaries - 笔记

## 本题会用到的前置知识点

- `&str`、`&[T]` 切片。
- 所有权与借用。
- 迭代器基础。

## 核心结论

- 只读文本优先用 `&str`，只读序列优先用 `&[T]`。
- 这类边界天然兼容 owned 与 borrowed 输入。
- 参数类型写成 `&str` / `&[T]` 时，函数表达的是“我只借用一段数据，不接管容器”。
- 学习阶段优先“清晰签名”，不要过早抽象。

## 需要补充看懂的知识

### `&String` 和 `&str` 的 API 含义不同

如果函数只需要读文本，优先写：

```rust
fn first_word(input: &str) -> &str {
	input.split_ascii_whitespace().next().unwrap_or("")
}
```

不要优先写：

```rust
fn first_word_less_flexible(input: &String) -> &str {
	input.split_ascii_whitespace().next().unwrap_or("")
}
```

`&String` 要求调用者必须有 `String`；`&str` 可以接收 `String` 的借用，也可以接收字符串字面量、字符串切片等。

```rust
let owned = String::from("rust lang");

first_word(&owned);      // &String 可以自动借成 &str
first_word("go now");    // 字面量本来就是 &str
first_word(&owned[..4]); // 字符串切片也是 &str
```

### `&Vec<T>` 和 `&[T]` 的 API 含义不同

如果函数只需要读一段元素，优先写：

```rust
fn sum_slice(values: &[i32]) -> i32 {
	values.iter().sum()
}
```

这样数组、`Vec`、切片都能用：

```rust
let arr = [1, 2, 3];
let vec = vec![4, 5];

sum_slice(&arr);
sum_slice(&vec);
sum_slice(&vec[0..1]);
```

`&Vec<i32>` 则把 API 绑定到了具体容器 `Vec`，明明只需要读元素，却让数组和其他切片形态不方便调用。

### 这不是“不要用 String / Vec”

`String` 和 `Vec<T>` 仍然很重要。它们适合表示“我拥有并可能修改这份数据”。

参数选择可以先按这个顺序判断：

- 只读一段文本：`&str`
- 只读一段元素：`&[T]`
- 需要拥有文本：`String`
- 需要拥有并增长/修改序列：`Vec<T>`

本题练的是函数边界，不是禁止使用 owned 类型。

### 返回值的生命周期来自输入

```rust
fn first_word(input: &str) -> &str {
	input.split_ascii_whitespace().next().unwrap_or("")
}
```

返回的 `&str` 通常是输入里的一个片段。它不能比输入活得更久。

```rust
let word;
{
	let s = String::from("rust lang");
	word = first_word(&s);
	// 这里 word 还能用
}
// 这里 s 已释放，word 不能继续使用
```

所以 `&str` / `&[T]` 边界虽然灵活，但仍然遵守借用规则：函数没有拿走所有权，也不能制造比输入更长寿的引用。

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
- 只读场景写成 `&String` / `&Vec<T>`，把 API 绑死在具体容器上。
- 为了“通用”引入不必要的复杂约束。
- 返回输入切片后，忘记返回值不能比输入活得更久。

## 回看问题

- 函数是否不必要地拿走了所有权？
- 当前签名对 `String`、字面量、切片是否都友好？
- 函数真正需要的是容器本身，还是容器里的一段数据？

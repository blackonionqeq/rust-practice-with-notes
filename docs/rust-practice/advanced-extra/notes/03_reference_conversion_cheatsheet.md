# 引用转换速查表：`&x` / `as_ref` / `as_mut`

## 核心心法：借整个值，还是借里面的值

```
&x              = 借用变量 x 本身
x.as_ref()      = 把 x 转成“共享引用视图”
x.as_mut()      = 把 x 转成“可变引用视图”
x.as_deref()    = as_ref 后再 deref 一层
x.as_deref_mut() = as_mut 后再 deref 一层
```

最容易混的地方是：`&x` 通常只是把外层包起来，而 `as_ref()` / `as_mut()` 经常会把外层容器里的值借出来。

---

## 速查表

| 我想做什么 | 用什么 |
|---|---|
| 普通只读借用整个变量 | `&x` |
| 普通可变借用整个变量 | `&mut x` |
| `Option<T>` 不移动里面的 `T`，只读访问 | `opt.as_ref()` |
| `Option<T>` 不移动里面的 `T`，可变访问 | `opt.as_mut()` |
| `Result<T, E>` 不移动里面的值，只读访问 | `res.as_ref()` |
| `Result<T, E>` 不移动里面的值，可变访问 | `res.as_mut()` |
| `Option<String>` 直接变成 `Option<&str>` | `opt.as_deref()` |
| `Option<String>` 直接变成 `Option<&mut str>` | `opt.as_deref_mut()` |
| 函数参数想同时接受 `String`、`&str` 等 | `impl AsRef<str>` 或 `T: AsRef<str>` |
| `HashMap<String, V>` 用 `&str` 查 key | `Borrow` 相关能力，通常直接 `map.get("key")` |
| 模式匹配里避免 move | `ref x` / `ref mut x` |

> 标准库里没有 `as_mut_ref()` 这个高频方法。通常你要找的是 `as_mut()`。

---

## 1. `&x` / `&mut x`：借整个变量

```rust
let name = String::from("Alice");

let a: &String = &name;
let b: &str = &name; // &String 可以自动 deref 成 &str

println!("{a}");
println!("{b}");
```

`&name` 的直接含义是“借用这个 `String` 本身”。如果目标类型是 `&str`，编译器可以通过 deref coercion 把 `&String` 调整成 `&str`。

可变借用同理：

```rust
let mut name = String::from("Alice");

let s: &mut String = &mut name;
s.push_str(" Smith");
```

---

## 2. `Option::as_ref()`：`Option<T>` → `Option<&T>`

```rust
let name = Some(String::from("Alice"));

let borrowed: Option<&String> = name.as_ref();

println!("{borrowed:?}");
println!("{name:?}"); // name 仍然可用
```

对比 `&name`：

```rust
let name = Some(String::from("Alice"));

let a: &Option<String> = &name;
let b: Option<&String> = name.as_ref();
```

这里差别非常关键：

```
&name         -> &Option<String>   // 借整个 Option
name.as_ref() -> Option<&String>   // 借 Option 里面的 String
```

---

## 3. `Option::as_mut()`：`Option<T>` → `Option<&mut T>`

```rust
let mut name = Some(String::from("Alice"));

if let Some(s) = name.as_mut() {
    s.push_str(" Smith");
}

println!("{name:?}"); // Some("Alice Smith")
```

`as_mut()` 的重点是：不取走 `String`，只是在 `Some` 的时候给你一个 `&mut String`。

---

## 4. `Result::as_ref()` / `Result::as_mut()`

`Result` 和 `Option` 的思路一样。

```rust
let result: Result<String, String> = Ok("hello".to_string());

let borrowed: Result<&String, &String> = result.as_ref();

println!("{borrowed:?}");
println!("{result:?}"); // result 仍然可用
```

可变版本：

```rust
let mut result: Result<String, String> = Ok("hello".to_string());

if let Ok(s) = result.as_mut() {
    s.push('!');
}

println!("{result:?}"); // Ok("hello!")
```

---

## 5. `as_deref()`：借出来后再 deref 一层

`as_deref()` 最常见的用途：把 `Option<String>` 变成 `Option<&str>`。

```rust
let name = Some(String::from("Alice"));

let a: Option<&String> = name.as_ref();
let b: Option<&str> = name.as_deref();
```

对比：

```
name.as_ref()   -> Option<&String>
name.as_deref() -> Option<&str>
```

所以如果你只是想读取字符串内容，`as_deref()` 经常比 `as_ref()` 更顺手：

```rust
fn greet(name: Option<&str>) {
    match name {
        Some(name) => println!("hello, {name}"),
        None => println!("hello"),
    }
}

let name = Some(String::from("Alice"));
greet(name.as_deref());
```

---

## 6. `as_deref_mut()`：可变版

```rust
let mut name = Some(String::from("alice"));

if let Some(s) = name.as_deref_mut() {
    s.make_ascii_uppercase();
}

println!("{name:?}"); // Some("ALICE")
```

它可以理解成：

```
Option<String>
  -> as_mut() 得到 Option<&mut String>
  -> deref_mut 得到 Option<&mut str>
```

---

## 7. `AsRef<T>`：函数参数里的“便宜引用转换”

`AsRef` 是 trait，核心方法是：

```rust
fn as_ref(&self) -> &T;
```

它适合表达：这个函数只需要某种引用视图，不关心调用方原本拿的是 `String`、`&str`、`PathBuf` 还是 `&Path`。

```rust
fn is_alice<S: AsRef<str>>(name: S) -> bool {
    name.as_ref() == "Alice"
}

assert!(is_alice("Alice"));
assert!(is_alice(String::from("Alice")));
```

路径场景也很常见：

```rust
use std::path::{Path, PathBuf};

fn print_path<P: AsRef<Path>>(path: P) {
    println!("{}", path.as_ref().display());
}

print_path("Cargo.toml");
print_path(PathBuf::from("src/main.rs"));
```

> 函数参数如果只是读字符串，优先 `&str`。只有想同时接收多种拥有者/引用类型，并且函数内部确实需要统一转成引用时，再考虑 `AsRef<str>`。

---

## 8. `Borrow<T>`：集合查找里更常见

日常不太需要手写 `borrow()`，但你会受益于它。例如 `HashMap<String, V>` 可以用 `&str` 查找：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 10);

assert_eq!(scores.get("Alice"), Some(&10));
```

这里 key 的真实类型是 `String`，但查找时可以传 `"Alice"`。这类能力主要由标准库集合和 `Borrow` 相关约束配合完成。

粗略记忆：

```
AsRef  -> 便宜地拿引用视图，常用于函数参数适配
Borrow -> 借用成等价的查找形式，常见于 HashMap / HashSet
```

---

## 9. `Deref`：平时少直接写 `.deref()`

`Deref` 是很多自动转换背后的机制。比如 `String` 可以 deref 成 `str`，所以 `&String` 经常能自动用在需要 `&str` 的地方。

```rust
fn print_str(s: &str) {
    println!("{s}");
}

let name = String::from("Alice");
print_str(&name); // 自动从 &String 转成 &str
```

你通常不用写：

```rust
use std::ops::Deref;

let name = String::from("Alice");
let s: &str = name.deref();
```

知道它存在即可。日常代码里，优先写 `&name`、`name.as_str()` 或 `name.as_deref()`。

---

## 10. `ref` / `ref mut`：模式匹配里借用字段

`ref` 不是方法，是模式匹配语法。

```rust
let name = Some(String::from("Alice"));

match name {
    Some(ref s) => println!("{s}"),
    None => {}
}

println!("{name:?}"); // name 没有被 move
```

不过很多时候，用 `as_ref()` 更直观：

```rust
let name = Some(String::from("Alice"));

match name.as_ref() {
    Some(s) => println!("{s}"),
    None => {}
}

println!("{name:?}");
```

可变借用：

```rust
let mut name = Some(String::from("Alice"));

match name {
    Some(ref mut s) => s.push('!'),
    None => {}
}

println!("{name:?}");
```

---

## 高频对比

### `&Option<T>` vs `Option<&T>`

```rust
let x = Some(10);

let a: &Option<i32> = &x;
let b: Option<&i32> = x.as_ref();
```

```
&Option<T>  = 借整个盒子
Option<&T>  = 盒子还在，但里面放的是引用
```

### `Option<&String>` vs `Option<&str>`

```rust
let name = Some(String::from("Alice"));

let a: Option<&String> = name.as_ref();
let b: Option<&str> = name.as_deref();
```

```
Option<&String> = 还保留 String 这个具体类型
Option<&str>    = 只要字符串切片视图，更通用
```

### `&String` vs `&str`

```rust
fn takes_string_ref(s: &String) {}
fn takes_str(s: &str) {}

let name = String::from("Alice");

takes_string_ref(&name);
takes_str(&name);
takes_str(name.as_str());
```

函数参数里通常更推荐 `&str`，因为它能接收 `String`、`&str`、字符串切片和字面量。

---

## 实战判断规则

1. 函数要 `&T`，你手里有 `T`：先试 `&x`。
2. 函数要 `&mut T`，你手里有 `mut x`：用 `&mut x`。
3. 你手里是 `Option<T>` / `Result<T, E>`，还想保留原值：用 `as_ref()` 或 `as_mut()`。
4. 你手里是 `Option<String>`，目标是 `Option<&str>`：用 `as_deref()`。
5. 函数参数想支持多种路径类型：常用 `impl AsRef<Path>`。
6. 函数参数只读字符串：优先 `&str`，不要急着写 `impl AsRef<str>`。
7. 模式匹配里不想 move：可以用 `ref` / `ref mut`，但 `as_ref()` / `as_mut()` 往往更清楚。

---

## 常见坑

- **以为 `&opt` 和 `opt.as_ref()` 一样**：不一样，前者是 `&Option<T>`，后者是 `Option<&T>`。
- **找 `as_mut_ref()`**：标准库常用的是 `as_mut()`。
- **函数参数写成 `&String`**：多数时候应写 `&str`。
- **滥用 `AsRef<str>`**：只读字符串参数用 `&str` 通常更简单；泛型参数会让签名更复杂。
- **忘了 `as_ref()` 不会 clone**：它只是借用，不复制内部值。
- **`as_deref()` 不是字符串专用**：只要内部类型能 deref，就可能有用；但日常最常见是 `String -> str`、`PathBuf -> Path`。

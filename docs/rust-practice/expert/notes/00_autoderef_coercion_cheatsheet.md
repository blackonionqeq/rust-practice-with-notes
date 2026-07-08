# Rust 自动解引用 / 自动借用 / Coercion 速查表

## 核心心法：不是到处自动转

Rust 确实会在一些位置自动帮你“补几步”，但它不是任意类型转换。

重点记住三类位置：

| 位置 | 常见自动行为 |
|---|---|
| 方法调用 | 自动借用、自动解引用 |
| 字段访问 | 自动解引用 |
| 有明确目标类型的位置 | deref coercion、引用到裸指针 coercion 等 |

如果没有目标类型，或者转换会改变所有权语义，Rust 通常不会替你猜。

---

## 1. 方法调用：自动借用 + 自动解引用

方法调用是最宽松、最常见的地方。

```rust
let mut s = String::from("hi");

s.push_str("!"); // 自动变成类似 String::push_str(&mut s, "!")
println!("{}", s.len()); // String 可通过 Deref 当作 str 使用
```

这里可能同时发生两件事：

- `s.push_str(...)` 自动借成 `&mut s`
- `String` 通过 `Deref<Target = str>` 找到 `str` 上的方法

智能指针也常见：

```rust
let name = Box::new(String::from("rust"));

println!("{}", name.len()); // Box<String> -> String -> str
```

记忆：点号调用方法时，Rust 会积极尝试把接收者调成方法需要的样子。

---

## 2. 字段访问：自动解引用

字段访问也会自动解引用。

```rust
struct User {
    name: String,
}

let user = User {
    name: String::from("Alice"),
};

let r = &user;
println!("{}", r.name); // &User 自动解到 User
```

`Box<T>` 这类实现了 `Deref` 的类型也类似：

```rust
let user = Box::new(User {
    name: String::from("Alice"),
});

println!("{}", user.name); // Box<User> 自动 deref 到 User
```

注意：字段访问能自动解引用，不代表会自动移动被引用对象里的字段。实际所有权规则仍然照常生效。

---

## 3. 函数参数：有目标类型时会 coercion

函数参数位置有明确的目标类型，所以 Rust 可以做一些安全的 coercion。

### `&String` -> `&str`

```rust
fn print_str(s: &str) {
    println!("{s}");
}

let s = String::from("hello");
print_str(&s); // &String -> &str
```

原因是：

```rust
String: Deref<Target = str>
```

### `&Vec<T>` -> `&[T]`

```rust
fn sum(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

let nums = vec![1, 2, 3];
println!("{}", sum(&nums)); // &Vec<i32> -> &[i32]
```

### `&T` -> `*const T`

这就是 71 题里的情况：

```rust
unsafe fn read_i32(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

let x = 234;
let result = unsafe { read_i32(&x) };
```

`read_i32` 的参数已经写死是 `*const i32`，所以 `&x` 可以自动 coercion 成 `*const i32`。

等价于更显式的写法：

```rust
let ptr = &x as *const i32;
let result = unsafe { read_i32(ptr) };
```

这一步转换本身不是危险操作；危险的是函数内部对裸指针做解引用。

---

## 4. `let` / 返回值：目标类型明确时也可能 coercion

只要左边或返回类型给出了明确目标类型，Rust 也可以做 coercion。

```rust
let s = String::from("hello");
let r: &str = &s; // &String -> &str
```

返回值也类似：

```rust
fn as_str(s: &String) -> &str {
    s // &String -> &str
}
```

如果没有目标类型，Rust 不会随便把一个值改成另一个类型。

---

## 5. 不会自动发生的事

### 不会自动解引用裸指针

```rust
let x = 1;
let p = &x as *const i32;

// println!("{}", *p); // 不行：裸指针解引用必须放进 unsafe
println!("{}", unsafe { *p });
```

`&T -> *const T` 可以安全产生裸指针；但 `*ptr` 读取内存需要调用者保证非空、对齐、已初始化、生命周期有效等条件。

### 不会自动做任意数字转换

```rust
let x: i32 = 1;
// let y: i64 = x; // 不行
let y: i64 = x.into();
```

### 不会自动替你选择会改变语义的转换

```rust
let s = String::from("hello");

// let bytes: Vec<u8> = s; // 不行
let bytes = s.into_bytes();
```

这类转换可能移动所有权、重新解释数据或改变 API 含义，Rust 要求你显式写出来。

---

## 和 71 题的关系

71 题的核心不是“怎么把引用变裸指针”，而是：

- `unsafe fn` 把安全前提交给调用者
- 调用点的 `unsafe {}` 表示调用者确认这次满足合约
- 真正危险的是裸指针解引用，不是从引用生成裸指针

所以这两种写法都能编译：

```rust
let result = unsafe { read_i32(&x) };
```

```rust
let ptr = &x as *const i32;
let result = unsafe { read_i32(ptr) };
```

练习里更推荐第二种，因为它把“从活着的局部变量派生裸指针”这一步展示得更清楚。

---

## 速记口诀

> 点号调用最积极：自动借用、自动解引用。  
> 字段访问会解引用，但所有权规则不放松。  
> 参数、let、返回值有目标类型，才可能 coercion。  
> 裸指针能自动生成，不能自动安全解引用。  
> 会改变语义的转换，自己显式写。

# String / &str 方法速查表（日常高频）

## 核心心法：视图 vs 所有者

```
&str   = 字符串的"借用视图"，不拥有数据，通常存在于字面量或其他 String 的切片
String = 堆上的拥有者，可增长，可修改
```

**关键规则：**
- `&String` 可以自动 deref 成 `&str`（因此接受 `&str` 的函数同样接受 `&String`）。
- `&str` **不能**自动变成 `String`（需要显式 `.to_string()` 或 `String::from(...)`）。
- **函数参数优先用 `&str`**，而不是 `&String`——更通用，接受面更广。

---

## 速查表

| 我想做什么 | 用什么 |
|---|---|
| 按分隔符拆分 | `.split(pat)` |
| 按行拆分 | `.lines()` |
| 按空白拆分（自动 trim） | `.split_whitespace()` |
| 去掉首尾空白 | `.trim()` |
| 检查是否以...开头 / 结尾 | `.starts_with(pat)` / `.ends_with(pat)` |
| 检查是否包含 | `.contains(pat)` |
| 替换子串 | `.replace(from, to)` |
| 遍历字符 | `.chars()` |
| 遍历字节 | `.bytes()` |
| 解析成数字/类型 | `.parse::<T>()` |
| 转大写 / 小写 | `.to_uppercase()` / `.to_lowercase()` |
| 取子串 | `&s[start..end]`（字节索引，须在 char 边界）|
| 字符数 | `.chars().count()` |
| 字节长度 | `.len()` |
| 是否为空 | `.is_empty()` |
| 追加字符串 | `s.push_str("...")` |
| 追加单个字符 | `s.push('x')` |
| 拼接多个 | `format!("{a}{b}")` |
| 从 Vec<String> 拼接 | `v.join(", ")` |
| &str → String | `.to_string()` 或 `String::from(s)` |
| String → &str | `&s` 或 `s.as_str()` |

---

## `&str` 高频方法

### 1. `split` — 按分隔符拆分（返回迭代器）

```rust
let parts: Vec<&str> = "a,b,c".split(',').collect(); // ["a", "b", "c"]
let parts: Vec<&str> = "one  two".split_whitespace().collect(); // ["one", "two"]
let lines: Vec<&str> = "a\nb\nc".lines().collect();  // ["a", "b", "c"]
```

> `split_whitespace` 自动 trim 并处理多个连续空白，比 `split(' ')` 更健壮。

### 2. `trim` 系列

```rust
"  hello  ".trim()        // "hello"
"  hello  ".trim_start()  // "hello  "
"  hello  ".trim_end()    // "  hello"
"##tag##".trim_matches('#') // "tag"（自定义字符）
```

### 3. `starts_with` / `ends_with` / `contains`

```rust
"hello".starts_with("hel")  // true
"hello".ends_with("lo")     // true
"hello".contains("ell")     // true
```

### 4. `replace` / `replacen`

```rust
"aaa".replace('a', "b")      // "bbb"（全部替换）
"aaa".replacen('a', "b", 2)  // "bba"（只替换前 2 次）
```

### 5. `chars` — 安全地遍历 Unicode 字符

```rust
"héllo".chars().count()       // 5（正确的字符数）
"héllo".len()                 // 6（字节数，UTF-8 中 é 占 2 字节）

let upper: String = "hello".chars().map(|c| c.to_uppercase().next().unwrap()).collect();
```

> **用 `.len()` 得到字节数，用 `.chars().count()` 得到字符数。** 对 ASCII 一样，对 Unicode 不同。

### 6. `parse::<T>()` — 字符串解析

```rust
let n: i32 = "42".parse().unwrap();                   // 类型标注在左侧
let n = "42".parse::<i32>().unwrap();                  // 或 turbofish
let result: Result<i32, _> = "bad".parse();            // 失败时用 Result 处理
```

### 7. `find` / `contains` — 查找位置

```rust
"hello".find('l')          // Some(2)（第一次出现的字节位置）
"hello".rfind('l')         // Some(3)（从右找）
```

---

## `String` 高频方法

### 8. 构造

```rust
String::new()               // 空字符串
String::from("hello")       // 从 &str
"hello".to_string()         // 同上（习惯上哪个都行，一致就好）
format!("{a} {b}")          // 拼接，不消耗所有权
```

### 9. 追加

```rust
let mut s = String::from("hello");
s.push(' ');                // 追加单个 char
s.push_str("world");        // 追加 &str
s.extend(["!", "!"]);       // 追加多个 &str
```

### 10. `join` — 从 Vec 拼接

```rust
let v = vec!["a", "b", "c"];
let s = v.join(", ");   // "a, b, c"

let nums = vec![1, 2, 3];
let s = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("-");
// "1-2-3"
```

---

## 类型转换速查

```
字面量 "hello"       → &str（天然就是）
&str → String        → "hello".to_string()  或  String::from("hello")
String → &str        → &s  或  s.as_str()  或  &s[..]
String → 消耗        → 直接传入接受 String 的函数
&String → &str       → 自动 deref，无需显式转换
Vec<u8> → String     → String::from_utf8(v)  （可能失败）
String → Vec<u8>     → s.into_bytes()
```

---

## 常见坑

- **`s.len()` 返回字节数，不是字符数**——中文、emoji 用 `.chars().count()`。
- **`&s[0..1]` 可能 panic**——如果 0..1 不在 UTF-8 字符边界上。安全做法：先用 `char_indices()` 找边界。
- **`split(' ')` vs `split_whitespace()`**——前者遇到多个空格会产生空字符串，后者不会。
- **`+` 拼接 String 会移走所有权**：`let s3 = s1 + &s2;` 之后 `s1` 不可用。多段拼接用 `format!`。
- **函数签名写 `s: &String` 不如写 `s: &str`**——`&str` 同时接受 `&String`、字面量、切片，更通用。

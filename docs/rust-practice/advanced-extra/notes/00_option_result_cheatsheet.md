# Option / Result 方法速查表（日常高频）

## 核心心法：记词根，不背方法

`Option` / `Result` 的方法不是随机冒出来的，而是几个"词根"组合出来的。记住下面 6 个词根，90% 的方法都能猜出七八成：

| 词根 | 含义 |
|---|---|
| `map` | 变换**里面的值**（结构不变）|
| `and_then` | 变换，且这一步**本身也可能失败**（自动摊平）|
| `or` | 没值时的**备用方案** |
| `unwrap` | 把值**拆出来** |
| `is_` | 判断（返回 `bool`）|
| `ok_` / `err` | Option ↔ Result 之间的**桥** |

- 知道 `or` = 备用 → `unwrap_or`、`unwrap_or_else`、`unwrap_or_default`、`ok_or`、`or_else` 全是同一家。
- 知道 `map` = 变换 → `map`、`map_err`、`map_or` 一目了然。

---

## 日常 10 个高频方法

下面这些覆盖了写业务代码约 90% 的场景。

### 1. `map` — 变换成功值
```rust
let n: Option<i32> = Some(5);
let doubled = n.map(|x| x * 2);   // Some(10)
// Result 的 map 只变换 Ok 里的值，Err 原样透传
```

### 2. `and_then` — 链式（闭包也返回 Option/Result）
```rust
// 闭包也返回 Option，用 and_then 自动摊平，避免 Option<Option<T>>
let port: Option<u16> = env::var("PORT")
    .ok()
    .and_then(|s| s.parse().ok());
```
> 记忆：闭包会失败 / 可能返回 `None` → 用 `and_then` 而不是 `map`。

### 3. `unwrap_or` 系列 — 给备用值（不 panic）
```rust
x.unwrap_or(0);                       // 备用是现成值（立即求值）
x.unwrap_or_else(|| load_default());  // 备用要算 → 用闭包（惰性）
x.unwrap_or_default();                // 备用 = 类型的 Default
```

### 4. `?` — 早退传播错误（语法，不是方法）
```rust
fn read() -> Result<String, std::io::Error> {
    let s = std::fs::read_to_string("a.txt")?; // Err 时自动 return
    Ok(s)
}
```

### 5. `ok_or` — Option 变 Result（给 None 一个错误）
```rust
let user: Result<&str, &str> = find(id).ok_or("not found");
```

### 6. `ok()` / `err()` — Result 变 Option（丢掉另一半）
```rust
let ok_val: Option<String> = env::var("X").ok(); // Err 变 None
```

### 7. `map_err` — 变换错误（Result 专用）
```rust
result.map_err(|e| format!("bad: {e}"))
```

### 8. `is_some/is_ok` / `is_none/is_err` — 判断
```rust
if opt.is_some() { /* ... */ }   // 返回 bool
```
> 注意：优先用 `match` / `if let` / `let-else` 直接拿值，少写 `is_xxx()` 然后再 `unwrap()`。

### 9. `or` / `or_else` — 多个来源按优先级取值
```rust
// 第一个为 None 才看第二个；用 or_else 才是惰性
let v = cli_arg.or_else(|| env::var("FALLBACK").ok());
```

### 10. `as_ref` / `cloned` / `copied` — 类型搬运（打借检查器）
```rust
let r: Option<&T> = owned.as_ref();  // Option<T>  -> Option<&T>
let c: Option<T> = ref_opt.cloned(); // Option<&T> -> Option<T>（需要 Clone）
let c: Option<T> = ref_opt.copied(); // 同上，但只用于 Copy 类型（更快）
```

---

## 方法 vs 语法，别搞混

下面这些**不是 Option/Result 的方法**，是 Rust 语言自带的，属于另一条学习线（模式匹配 / 错误传播）：

| 写法 | 作用 |
|---|---|
| `?` | 早退传播错误（见上）|
| `if let Some(x) = opt {}` | 只关心一种情况时，比 `match` 简洁 |
| `let Some(x) = opt else { return };` | 解构取值，失败就走 `else` 分支（取代 `is_none` + `unwrap`）|
| `match opt { Some(x) => ..., None => ... }` | 完整覆盖两种情况 |

---

## 怎么查、怎么学

1. **VS Code + rust-analyzer**：写完 `. ` 自动补全会列出全部方法 + 签名 + 文档，扫一眼名字基本就懂。
2. **官方文档**（每个方法都带例子，资深的人也天天翻）：
   - Option: <https://doc.rust-lang.org/std/option/enum.Option.html>
   - Result: <https://doc.rust-lang.org/std/result/enum.Result.html>
3. **遇到陌生方法，先把名字拆成词根**（map? or? and? unwrap?），通常能猜出意思，再查文档确认。

---

## 速记口诀

> 取值要备用 → `unwrap_or`
> 链式会失败 → `and_then`
> 变成功值 → `map`，变错误 → `map_err`
> Option 要错误 → `ok_or`，Result 要丢弃 → `ok()`
> 能早退就早退 → `?`

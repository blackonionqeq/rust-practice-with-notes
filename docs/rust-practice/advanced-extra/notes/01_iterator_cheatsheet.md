# Iterator 方法速查表（日常高频）

## 核心心法：懒执行 + 终止操作

```
集合 → .iter() → [适配器链：全是懒的] → 终止操作（才真正执行）
```

- **适配器**（返回另一个 Iterator）：`map`、`filter`、`flat_map`、`enumerate`、`zip`、`take`、`skip`、`chain`
- **终止操作**（消耗 Iterator，产出结果）：`collect`、`for`、`count`、`sum`、`any`/`all`/`find`、`max`/`min`

> 写了一串 `.map().filter()` 之后什么都没发生——正常，因为你还没有终止操作。

---

## 速查表

| 我想做什么 | 用什么 |
|---|---|
| 把每个值变换一遍 | `.map(|x| ...)` |
| 只保留满足条件的 | `.filter(|x| ...)` |
| 变换 + 过滤 None（一步搞定） | `.filter_map(|x| ...)` |
| 把嵌套集合摊平 | `.flat_map(|x| ...)` |
| 带上序号 | `.enumerate()` |
| 合并两个迭代器（配对） | `.zip(other)` |
| 拼接两个迭代器（串联） | `.chain(other)` |
| 只取前 N 个 | `.take(n)` |
| 跳过前 N 个 | `.skip(n)` |
| 是否有一个满足条件 | `.any(|x| ...)` |
| 是否全部满足条件 | `.all(|x| ...)` |
| 找到第一个满足条件的 | `.find(|x| ...)` |
| 找到第一个满足条件的下标 | `.position(|x| ...)` |
| 计数 | `.count()` |
| 求和 | `.sum::<T>()` |
| 最大 / 最小值 | `.max()` / `.min()` |
| 收集成 Vec | `.collect::<Vec<_>>()` |
| 收集成 HashMap | `.collect::<HashMap<_, _>>()` |
| 就地遍历（不收集） | `for x in iter {}` |

---

## 方法详解（带示例）

### 1. `map` — 变换每个值

```rust
let doubled: Vec<i32> = vec![1, 2, 3].iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

### 2. `filter` — 只保留满足条件的

```rust
let evens: Vec<_> = (0..10).filter(|x| x % 2 == 0).collect();
// [0, 2, 4, 6, 8]
```

### 3. `filter_map` — 变换 + 过滤 None（一步）

```rust
// 等价于 .map(...).filter(Option::is_some).map(Option::unwrap)
let nums: Vec<i32> = vec!["1", "x", "3"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();
// [1, 3]
```

> 遇到"变换后还要跳过失败/None"的场景，优先 `filter_map` 而不是 `map` + `filter`。

### 4. `flat_map` — 摊平嵌套

```rust
let words: Vec<&str> = vec!["hello world", "foo bar"]
    .iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
// ["hello", "world", "foo", "bar"]
```

### 5. `enumerate` — 带序号

```rust
for (i, v) in ["a", "b", "c"].iter().enumerate() {
    println!("{i}: {v}");
}
```

### 6. `zip` — 两个迭代器配对

```rust
let keys = ["x", "y", "z"];
let vals = [1, 2, 3];
let pairs: Vec<_> = keys.iter().zip(vals.iter()).collect();
// [("x", 1), ("y", 2), ("z", 3)]
```

### 7. `chain` — 两个迭代器串联

```rust
let all: Vec<i32> = vec![1, 2].into_iter().chain(vec![3, 4]).collect();
// [1, 2, 3, 4]
```

### 8. `any` / `all` / `find`

```rust
let has_neg = vec![1, -2, 3].iter().any(|x| *x < 0);  // true
let all_pos = vec![1, 2, 3].iter().all(|x| *x > 0);   // true
let first_neg = vec![1, -2, -3].iter().find(|x| **x < 0); // Some(-2)
```

### 9. `count` / `sum` / `max` / `min`

```rust
let n = (1..=5).count();            // 5
let s: i32 = (1..=5).sum();         // 15
let m = vec![3, 1, 4, 1, 5].iter().max(); // Some(5)
```

### 10. `collect` — 收集成集合（类型标注在左侧或 turbofish）

```rust
let v: Vec<_> = iter.collect();                    // Vec，让编译器推断元素类型
let s: String = chars.collect();                   // 从 char 迭代器收集成 String
let m: HashMap<_, _> = pairs.into_iter().collect(); // 从 (k, v) 收集成 HashMap
```

---

## `iter()` / `iter_mut()` / `into_iter()` — 最常见迷惑点

| 方法 | 产生 | 原集合之后 | 用途 |
|---|---|---|---|
| `.iter()` | `Iterator<Item = &T>` | 仍可用（借用） | 只读遍历 |
| `.iter_mut()` | `Iterator<Item = &mut T>` | 仍可用（可变借用） | 就地修改 |
| `.into_iter()` | `Iterator<Item = T>` | 被消耗（转移所有权） | 取出值，转换集合 |

```rust
let v = vec![1, 2, 3];

// iter(): 借用，v 还能用
for x in v.iter() { println!("{x}"); }
println!("{v:?}"); // 仍可用

// into_iter(): 消耗，v 不能再用
for x in v.into_iter() { println!("{x}"); }
// println!("{v:?}"); // 编译错误：已移走

// for x in &v       等价于 v.iter()
// for x in &mut v   等价于 v.iter_mut()
// for x in v        等价于 v.into_iter()
```

---

## Vec 补充：高频变异操作

```rust
v.retain(|x| *x > 0);          // 就地过滤（保留满足条件的）
v.sort_by_key(|x| x.score);    // 按字段排序
v.dedup();                      // 去掉相邻重复（先 sort 再 dedup = 全局去重）
v.extend([4, 5, 6]);            // 批量追加
```

## HashMap 补充：entry API

```rust
use std::collections::HashMap;
let mut map: HashMap<&str, i32> = HashMap::new();

// 不存在时插入，存在时更新——最常用模式
*map.entry("score").or_insert(0) += 10;

// 不存在时插入默认值（等价于 or_insert(Default::default())）
map.entry("level").or_default();

// 不存在时用闭包计算初始值（适合初始值计算成本较高的场景）
map.entry("key").or_insert_with(|| expensive_default());
```

---

## 常见坑

- **`map` 里做了可能失败的操作** → 改用 `filter_map` 或 `flat_map`。
- **迭代器写完什么都没发生** → 缺终止操作（`collect`、`for`、`sum` 等）。
- **`iter()` 拿到的是 `&&T`（双重引用）** → 在 `filter` / `map` 闭包里加 `*x` 解引用，或用 `iter().copied()`（Copy 类型专用，直接拿到 `T`）。
- **`collect` 类型报错** → 在左侧显式标注 `let v: Vec<i32> = ...`，或在 `collect::<Vec<i32>>()` 里写 turbofish。

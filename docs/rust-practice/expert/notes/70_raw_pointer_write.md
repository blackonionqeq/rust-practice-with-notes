# 70 Raw Pointer Write - 笔记

## 本题会用到的前置知识点

- 可变引用与切片。
- 裸指针读（69 题）。

## 核心结论

- `*mut T` 允许写入，但你必须保证三件事：目标地址有效、正确对齐、没有其他引用同时访问同一位置。
- `// Safety:` 注释要写具体条件，不能写"trust me"或"it's fine"。
- 从 `&mut [T]` 派生的 `*mut T` 天然满足"无其他引用同时访问"——因为可变引用本身就保证了独占性。

### 现实中哪里会见到

裸指针写入常见于 FFI 的输出参数、底层容器往已分配内存中写元素、手写缓冲区/分配器、或性能敏感代码在外层已经证明边界和独占访问后直接写入。

普通业务代码里，优先用 `get_mut`、索引赋值、`iter_mut()` 或标准容器方法。只有当写入目标无法用普通 `&mut T` 安全表达，且你能证明地址有效并且没有别名时，才考虑 `*mut T`。

## 需要补充看懂的知识

### 写入比读取多一个风险：独占访问

读裸指针时，你主要证明“这里能读”：地址有效、对齐、已初始化、生命周期内。

写裸指针时，还要证明“这里能改”：同一位置没有其他活跃引用正在读写，尤其不能和 `&T` / `&mut T` 形成冲突。

```rust
fn negate_first(data: &mut [i32]) {
	// data 是可变借用，调用方在本函数运行期间不能再访问同一段切片。
}
```

所以 `&mut [i32]` 是本题最重要的证明来源。它给了函数对这段数据的独占访问权。

### `as_mut_ptr()` 只是拿地址

```rust
let p: *mut i32 = data.as_mut_ptr();
```

这行仍然是安全操作。它只是从可变切片取出指向首元素的裸指针。真正需要 unsafe 的是：

```rust
unsafe {
	*p = -(*p);
}
```

这里包含一次读 `*p` 和一次写 `*p = ...`。两者都依赖相同的安全证明：`p` 指向有效首元素，并且当前函数有独占访问权。

### 空切片检查证明“首元素存在”

```rust
if data.is_empty() { return; }
```

这行保证 `data.as_mut_ptr()` 指向的首元素确实存在。没有这个检查，`p` 可能只是一个不能解引用的 dangling 指针。

### 不要在 unsafe 前后制造别名

下面这种方向要避免：

```rust
// 错误思路：先拿到首元素的可变引用，又用裸指针改同一位置
// let first = &mut data[0];
// let p = data.as_mut_ptr();
// unsafe { *p = 10; }
// println!("{first}");
```

裸指针本身不受借用检查严格管理，但你仍然必须遵守 Rust 的 aliasing 规则。safe wrapper 的安全性来自“没有额外别名”这个事实。

## 核心示例

```rust
fn negate_first(data: &mut [i32]) {
    if data.is_empty() { return; }
    let p: *mut i32 = data.as_mut_ptr();
    // Safety: data is non-empty (checked above), so p points to a valid,
    // aligned i32. The &mut reference guarantees exclusive access.
    unsafe {
        *p = -(*p);
    }
}
```

理解重点：`&mut [i32]` 的存在本身就是"无 aliasing"的证明；你只需要再证明"非空"。

## 常见坑

- 同时持有 `&mut data[0]` 和 `*mut i32` 指向同一位置——这违反 aliasing 规则，是 UB。
- `// Safety:` 写成"checked above"而不指向具体的哪个检查。
- 读写混写在多个语句里，不确定哪个语句触发了 unsafe——本题只有一个写操作，保持简单。
- 忘记空切片时 `as_mut_ptr()` 不能解引用。
- 以为 `*mut T` 的“mut”可以绕开 Rust 的独占访问规则。

## 回看问题

- 为什么从 `&mut [i32]` 派生的 `*mut i32` 不会产生 aliasing 问题？
- 如果在 `unsafe` 块里先读后写（两个操作），Safety 注释应该写什么？
- 哪一行证明了 `p` 指向的首元素存在？

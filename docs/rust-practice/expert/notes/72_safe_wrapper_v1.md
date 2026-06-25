# 72 Safe Wrapper v1 - 笔记

## 本题会用到的前置知识点

- 裸指针读（69 题）。
- `unsafe fn` contract 概念（71 题）。
- `Option` 返回值。

## 核心结论

- **safe wrapper 模式**：在进入 `unsafe` 之前，在安全代码里完成所有前提检查，然后带着"这些条件已成立"的保证进入 `unsafe`。
- `// Safety:` 注释要指向具体的哪个检查，而不是泛泛说"已经检查过了"。
- 调用方看到的是普通 safe 函数，不需要知道内部有 unsafe——这就是封装的价值。

## 需要补充看懂的知识

### safe wrapper 是“责任转移”

第 69 题的版本类似这样：

```rust
fn first_byte(data: &[u8]) -> u8
```

它要求调用方保证 `data` 非空。调用方一旦传空切片，函数内部解引用首元素就是 UB。

第 72 题改成：

```rust
pub fn first_byte(data: &[u8]) -> Option<u8>
```

现在调用方可以传任何 `&[u8]`。函数内部负责检查空切片，并用 `None` 表示没有首元素。unsafe 的前提不再由调用方记住，而是由 wrapper 自己建立。

### safe 函数必须对所有合法输入都安全

safe 函数的承诺是：只要调用方能用普通 Rust 调用它，就不能触发 UB。

所以这个签名：

```rust
pub fn first_byte(data: &[u8]) -> Option<u8>
```

必须安全处理：

- 非空切片。
- 空切片。
- 字面量切片。
- `Vec<u8>` 的借用。

如果某个输入会让函数内部 UB，这个函数就不应该是 safe 签名。

### `Option` 是安全边界的一部分

`Option<u8>` 不只是业务返回值，它表达了不变量检查的结果：

- `None`：没有首元素，因此不进入 unsafe 解引用。
- `Some(byte)`：已经证明首元素存在，可以读取。

这比返回 `u8` 并要求调用者“别传空”更稳，因为类型把失败路径暴露出来了。

### `// Safety:` 要贴近 unsafe

推荐结构：

```rust
if data.is_empty() {
	return None;
}
let p = data.as_ptr();
// Safety: data is non-empty (checked above), so p points to the first byte.
let byte = unsafe { *p };
```

检查、指针创建、`// Safety:`、unsafe 操作之间距离越短，越容易审查证明链。

## 核心示例

```rust
pub fn first_byte(data: &[u8]) -> Option<u8> {
    if data.is_empty() {
        return None;
    }
    let p: *const u8 = data.as_ptr();
    // Safety: data is non-empty (checked above), so p is valid and non-null.
    let byte = unsafe { *p };
    Some(byte)
}
```

理解重点：`if data.is_empty()` 这一行就是整个安全保证的来源；`// Safety:` 注释只是把这个事实显式写出来。

## 常见坑

- 把 safe wrapper 写成 `unsafe fn`——这样调用方还是需要承担责任，封装的意义就消失了。
- `// Safety:` 写在 unsafe 块之后，或者写在离 unsafe 块很远的地方——注释要紧贴 unsafe 块。
- 检查和进入 unsafe 之间插入了可能改变状态的代码，使得"条件已成立"的保证失效。
- safe 函数对某些普通输入仍可能 UB。
- 返回类型没有表达失败路径，却把失败情况留给调用者猜。

## 回看问题

- 如果去掉 `if data.is_empty()` 检查，这个函数还能保持 safe 签名吗？
- 69 题里的 `first_byte` 和本题的 `first_byte` 有什么本质区别？
- 这个 wrapper 是否对所有合法 safe 输入都不会 UB？

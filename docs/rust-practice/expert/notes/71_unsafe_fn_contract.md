# 71 Unsafe Function Contract - 笔记

## 本题会用到的前置知识点

- unsafe 块与裸指针（68-70 题）。
- 文档注释语法（`///`）。

## 核心结论

- `unsafe fn` 向调用方声明：**你在调用之前必须自己保证这些前提条件，编译器不检查。**
- `# Safety` 文档注释是这份合约的正式书写位置。
- `unsafe fn` ≠ "函数内部有 unsafe 块"；它是一种 API 契约，告知调用者有义务。
- `unsafe fn` 的函数体里仍然应该把具体 unsafe 操作限制在小的 `unsafe {}` 块中。
- 一个函数是否是 `unsafe fn`，取决于调用者是否必须满足额外安全前提，不取决于函数内部有没有 unsafe 实现细节。

## 需要补充看懂的知识

### `unsafe fn` 是把责任放到调用点

```rust
unsafe fn read_i32(ptr: *const i32) -> i32 {
	unsafe { *ptr }
}
```

这个签名表达的是：调用者必须先保证 `ptr` 可安全读取。调用点必须写：

```rust
let value = unsafe { read_i32(ptr) };
```

这个 `unsafe {}` 不是装饰语法，而是调用者在说：“我已经检查过这次调用满足 `# Safety` 合约。”

### `# Safety` 要写可检查的条件

不够好的写法：

```rust
/// # Safety
/// Caller must ensure ptr is valid.
```

更好的写法：

```rust
/// # Safety
///
/// - `ptr` must be non-null.
/// - `ptr` must be properly aligned for `i32`.
/// - `ptr` must point to an initialized `i32`.
/// - The pointed-to memory must remain valid for the duration of this call.
```

“valid” 太宽泛，读者不知道要检查什么。合约要具体到调用者能逐条确认。

### unsafe 实现细节不一定要暴露给调用者

如果函数自己能检查所有前提，就不应该写成 `unsafe fn`：

```rust
pub fn first_byte(data: &[u8]) -> Option<u8> {
	if data.is_empty() {
		return None;
	}
	let p = data.as_ptr();
	// Safety: checked non-empty above.
	Some(unsafe { *p })
}
```

调用者不需要知道里面用了裸指针，因为函数已经在 safe 边界内处理了风险。这就是第 72 题的 safe wrapper。

### unsafe 函数体规则也要谨慎

即使函数本身是 `unsafe fn`，也建议把具体 unsafe 操作放在小的 `unsafe` 块里，并写清楚局部证明。

这样读代码时能分清：

- `# Safety`：调用者必须保证什么。
- `// Safety:`：当前函数内部这次 unsafe 操作为何成立。

## 两种注释形式对比

| 场景 | 写法 |
|---|---|
| `unsafe fn` 的 API 文档 | `/// # Safety` 文档注释（在函数外） |
| 普通函数内部的 unsafe 块 | `// Safety:` 行内注释（在 unsafe 块前） |

## 核心示例

```rust
/// # Safety
///
/// - `ptr` must be non-null.
/// - `ptr` must point to a valid, initialized `i32`.
/// - The pointed-to value must not be aliased by a `&mut i32` at call time.
unsafe fn read_i32(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

fn main() {
    let x: i32 = 99;
    // Correct: pointer derived from a live local variable.
    let result = unsafe { read_i32(&x as *const i32) };
    println!("{result}");

    // Incorrect (do not run): null pointer violates precondition 1.
    // let _ = unsafe { read_i32(std::ptr::null()) };
}
```

理解重点：调用 `unsafe fn` 时仍然需要 `unsafe {}` 块——调用点是显式承担责任的地方。

## 常见坑

- 写了 `unsafe fn` 但省掉 `# Safety` 文档——调用者无法知道需要满足什么条件。
- 以为 `unsafe fn` 调用时不需要 `unsafe {}` 块——仍然需要，否则编译报错。
- `# Safety` 写得太泛：说"确保指针有效"而不说"什么叫有效"（非空、对齐、生命周期内等）。
- 函数能自己检查前提，却仍写成 `unsafe fn`，把责任不必要地推给调用者。
- 把 `# Safety` 文档和 `// Safety:` 块注释混用。

## 回看问题

- 如果一个函数内部有 `unsafe {}` 块但函数签名不是 `unsafe fn`，调用方需要写 `unsafe {}` 吗？
- `# Safety` 里的前提条件，违反了会怎样？编译器会报错吗？
- 这个函数的额外安全前提到底只能由调用者知道，还是函数内部可以自己检查？

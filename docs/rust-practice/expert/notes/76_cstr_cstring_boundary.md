# 76 CStr and CString Boundary - 笔记

## 本题会用到的前置知识点

- `Result` 错误处理。
- Rust 字符串基础（`&str`、`String`）。

## 核心结论

- C 字符串以 `\0` 结尾，不允许内部含有 `\0`——这是与 Rust `&str` 的根本区别。
- `CString`：拥有所有权的 C 字符串（堆分配，末尾自动加 `\0`）。
- `&CStr`：借用的 C 字符串视图（类比 `&str` 之于 `String`）。
- 绝大多数转换是安全的，只有直接操作指针才需要 `unsafe`。

## 需要补充看懂的知识

### C 字符串没有单独保存长度

Rust 的 `String`/`&str` 逻辑上知道自己的长度，所以内容中可以有 `\0` 字节；它只是普通字节之一。

C 字符串通常只靠第一个 `\0` 判断结尾。因此 `"hel\0lo"` 在 C 侧会被当成 `"hel"`，后面的 `lo` 不属于这段字符串。`CString::new` 拒绝内部 null 字节，是为了避免 Rust 以为传出的是完整字符串，而 C 侧提前截断。

### `CString` 拥有内存，`&CStr` 只是视图

`CString` 类似 `String`：它负责分配和释放这段以 `\0` 结尾的内存。

`&CStr` 类似 `&str`：它不拥有内存，只是借用已有 C 字符串。只要原来的 `CString` 被 drop，通过 `as_ptr()` 拿到的裸指针就不能再被 C 侧保存或使用。

### 字节合法和 UTF-8 合法是两回事

C 字符串的核心要求是"以 null 结尾，内部没有 null"；它不天然要求内容是 UTF-8。

`CStr::to_str()` 会额外检查 UTF-8，所以返回 `Result<&str, Utf8Error>`。如果你只是想展示可能不是 UTF-8 的 C 字符串，可以用 `to_string_lossy()`，它会把非法 UTF-8 字节替换掉。

## 类型对应关系

| Rust | C 侧 |
|---|---|
| `CString` | `char *`（拥有） |
| `&CStr` | `const char *`（借用） |
| `CString::as_ptr()` | 传递给 C 函数的指针 |

## 核心示例

```rust
use std::ffi::{CStr, CString};

fn main() {
    // 创建 CString
    let cs = CString::new("hello").expect("no null bytes");

    // 借用为 &CStr
    let cstr: &CStr = cs.as_c_str();

    // 转回 &str
    let s: &str = cstr.to_str().expect("valid UTF-8");
    println!("{s}"); // hello

    // 含内部 null 字节时报错
    match CString::new("hel\0lo") {
        Ok(_) => unreachable!(),
        Err(e) => println!("error: {e}"),
    }
}
```

## 常见坑

- 用完 `CString` 后让其提前 drop，但 C 侧仍持有指针——悬空指针（典型的生命周期陷阱）。
- 忘记 `CString::new` 返回 `Result`，直接 `unwrap`——含 null 字节的输入会 panic。
- 混淆 `to_str()`（可能失败，要求 UTF-8）和 `to_string_lossy()`（不失败，替换非法字节）。

## 回看问题

- 为什么 C 字符串不能包含内部 `\0`？
- `CString` drop 后，之前通过 `as_ptr()` 拿到的指针还有效吗？

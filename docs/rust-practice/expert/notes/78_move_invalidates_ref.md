# 78 Move Invalidates Raw Pointer - 笔记

## 本题会用到的前置知识点

- 裸指针基础（69-70 题）。
- struct 定义与移动语义。

## 核心结论

- 在 Rust 中，**移动（move）= 把字节复制到新地址，然后旧地址不再有效**。
- 如果一个 struct 里有指向自身字段的裸指针，移动之后该指针仍然指向旧地址——而旧地址的数据已经失效。
- 这就是 self-referential 裸指针在移动后变成悬空指针的根本原因。

## 核心示例

```rust
struct Fragile {
    data: [i32; 3],
    ptr: *const i32,  // 打算指向 data[0]
}

fn main() {
    let mut a = Fragile { data: [1, 2, 3], ptr: std::ptr::null() };
    a.ptr = a.data.as_ptr();
    println!("a.data[0] at: {:p}", &a.data[0]);
    println!("a.ptr  =  {:p}", a.ptr);  // 相同

    let b = a;  // move: 字节复制到新栈地址
    println!("b.data[0] at: {:p}", &b.data[0]);
    println!("b.ptr  =  {:p}", b.ptr);  // 仍然是旧地址！

    // 此时 b.ptr 指向旧栈帧（a 的位置），读取它是 UB
    // unsafe { println!("{}", *b.ptr) }  // 不要运行这行
}
```

理解重点：两次打印的 `ptr` 地址相同，但 `data[0]` 的地址变了——指针已经悬空。

## 常见坑

- 在调试构建下，栈帧地址可能恰好相同（未被覆盖），让 UB 看起来"正常"——这是最危险的 bug 类型。
- 认为"只是复制了字节，数据还在"——数据在 b 里，但 ptr 的值是旧的，指向已失效的内存。

## 回看问题

- 如果 `data` 是堆上的 `Vec<i32>` 而不是栈上的 `[i32; 3]`，移动后 `ptr` 还会失效吗？（提示：Vec 移动后，堆数据地址不变，只有 Vec 元数据的栈地址变了）
- `Pin` 要解决的是什么——让数据不能被移动，还是让指针在移动后自动更新？

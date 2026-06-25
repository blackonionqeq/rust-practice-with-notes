# 77 Self-Referential Struct Problem - 笔记

## 本题会用到的前置知识点

- struct 定义与生命周期标注。
- 字符串切片。

## 核心结论

- Rust 借用检查器不允许一个 struct 同时持有数据和指向该数据的引用，因为移动 struct 会导致引用失效。
- 编译器报错的本质：`data` 的生命周期与 `self` 的生命周期是同一个对象，但 struct 的生命周期标注要求"引用比 struct 活得更长"，产生矛盾。
- **最简单的解法：用 index 代替引用。** 不存引用，存偏移量。

## 需要补充看懂的知识

### 生命周期参数描述的是外部借用，不是"字段之间的关系"

`struct SelfRef<'a> { data: String, view: &'a str }` 里的 `'a` 表示：`view` 借用的字符串切片来自某个在 struct 外部已经存在、并且至少活到 `'a` 的数据。

它不能表达"这个字段借用同一个 struct 的另一个字段"。Rust 的普通引用模型要求被借用的数据比引用活得更久，而 `data` 和 `view` 会一起被 drop，不存在 `data` 比 `view` 所在的整个 struct 更长寿这件事。

### 构造阶段也解决不了自引用

常见尝试是先有 `s: String`，取 `let view = &s[..]`，再返回 `SelfRef { data: s, view }`。问题是返回时 `s` 会被移动进 `data` 字段，原来指向局部变量 `s` 的引用不再能描述移动后的字段位置。

这不是语法问题，而是地址稳定性问题：只要值还能被移动，指向它内部的引用就可能失效。

### index 方案把"借用"推迟到方法调用时

`TokenView` 存的是 `start`/`end`，不是 `&str`。调用 `view(&self)` 时才临时生成 `&self.data[start..end]`，这个引用的生命周期绑定到当前这次 `&self` 借用。

所以它没有长期保存内部引用，也就不会遇到自引用 struct 的问题。

## 为什么会报错

```rust
struct SelfRef<'a> {
    data: String,
    view: &'a str,  // 'a 必须比 SelfRef 本身活得更长
}
// 问题：data 属于 SelfRef，不可能比 SelfRef 活得更长
```

理解重点：引用的生命周期语义要求"被引用对象比引用者更长寿"，self-reference 违反这一点。

## 核心示例（index 方案）

```rust
struct TokenView {
    data: String,
    start: usize,
    end: usize,
}

impl TokenView {
    fn new(data: String, start: usize, end: usize) -> Self {
        Self { data, start, end }
    }
    fn view(&self) -> &str {
        &self.data[self.start..self.end]
    }
}
```

理解重点：不存引用，存位置信息。`view()` 在调用时临时生成引用，生命周期绑定到 `&self`，没有问题。

## 常见坑

- 尝试用 `'self` 之类的方式表达"引用指向自身"——Rust 没有这个概念。
- 以为生命周期标注可以"绕过"借用检查——不能，它只是让编译器的推断更精确。

## 回看问题

- 如果 `data` 是 `&'static str` 而不是 `String`，会有问题吗？
- index 方案的缺点是什么？（提示：如果 `data` 被修改，index 可能失效）

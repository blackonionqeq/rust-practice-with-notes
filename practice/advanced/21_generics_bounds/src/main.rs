use std::fmt::Display;

// Rust 会自动把 prelude 里的常用类型、trait 和宏导入当前作用域。
// 例如 PartialOrd、Clone、Iterator、Option、Result、Vec、String、println! 等可以直接用。
// Display 不在 prelude 里，所以这里需要显式 use std::fmt::Display。
//
// 常见 trait 速查：
// PartialOrd: 支持 <、<=、>、>=，但不保证所有值都能比较，例如 f32/f64 的 NaN。
// Ord: 支持完整稳定排序，常用于 sort、BTreeMap、BTreeSet、BinaryHeap。
// PartialEq/Eq: 支持 ==、!=；Eq 表示等价关系更完整，常和 HashMap key 搭配。
// Clone/Copy: 支持复制值；Clone 可能有成本，Copy 是按位复制且不移动所有权。
// Display: 支持 "{}" 面向用户展示；Debug 支持 "{:?}" 面向调试。
// Hash: 支持哈希，HashMap/HashSet 的 key 通常需要 Eq + Hash。
// Iterator/IntoIterator: 支持 for 循环和 map/filter/collect 这类迭代器方法。
// From/Into/TryFrom/TryInto: 支持类型转换。
// Default: 支持创建默认值，例如 T::default()。

#[derive(Debug)]
struct Pair<T> {
    left: T,
    right: T,
}

fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

fn describe_pair<T: Display>(pair: &Pair<T>) -> String {
    format!("left: {}, right: {}", pair.left, pair.right)
}

fn larger_pair_value<T: PartialOrd>(pair: Pair<T>) -> T {
    larger(pair.left, pair.right)
}

fn main() {
    println!("larger(3,7): {}", larger(3, 7));
    println!("larger(\"rust\", \"go\"): {}", larger("rust", "go"));
    let pair = Pair { left: 10, right: 8 };
    println!("describe_pair(pair): {}", describe_pair(&pair));
    println!("larger_pair_value(pair): {}", larger_pair_value(pair));
}

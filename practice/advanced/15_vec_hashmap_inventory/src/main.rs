use std::collections::HashMap;

fn build_items() -> Vec<String> {
    // 另一种更“手动”的写法：先创建 Vec，再逐个 push。
    // 初学时这很好懂；熟悉迭代器后，下面的 map + collect 写法更简洁。
    //
    // let mut vec = Vec::new();
    // for item in ["book", "pen", "book", "notebook", "pen", "book"] {
    //     vec.push(item.to_string());
    // }
    // vec
    //
    // 也可以直接用 vec! 宏，但每个元素仍然要是 String：
    // vec![
    //     String::from("book"),
    //     String::from("pen"),
    //     String::from("book"),
    //     String::from("notebook"),
    //     String::from("pen"),
    //     String::from("book"),
    // ]

    // 字符串字面量的类型是 &'static str，也就是“指向程序内置字符串的引用”。
    // 所以下面这个数组的类型可以理解为： [&'static str; 6]
    ["book", "pen", "book", "notebook", "pen", "book"]
        // into_iter() 会把数组变成迭代器。
        // 这里每次迭代拿到的是一个 &str，例如 "book"。
        .into_iter()
        // 不能省略这一步：函数要求返回 Vec<String>，但当前迭代器元素是 &str。
        // Rust 不会隐式把 &str 转成 String，必须显式转换。
        // 常见写法有三种：
        //   .map(|s| s.to_string())
        //   .map(|s| String::from(s))
        //   .map(String::from)
        .map(String::from)
        // collect() 会根据函数返回类型 Vec<String> 推断收集目标。
        // 如果没有返回类型帮助推断，也可以显式写：.collect::<Vec<String>>()
        .collect()
}

// 参数写成 &[String] 而不是 &Vec<String>，因为切片更泛用。
// Vec<String>、数组 [String; N]、以及 Vec 的一部分切片都可以作为 &[String] 使用。
fn count_items(items: &[String]) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    items.iter().for_each(|item| {
        // items.iter() 迭代出来的是 &String，不是 String。
        // map 的类型是 HashMap<String, u32>，key 必须是拥有所有权的 String。
        // entry() 可能会插入新 key，所以这里需要 item.clone() 给 HashMap 一份自己的 String。
        //
        // 注意：get() / get_mut() 只是查询，可以用 &str 查 HashMap<String, _>：
        //   map.get("book")
        //   map.get_mut(item.as_str())
        // 但 entry() 可能插入，因此需要 String：
        //   map.entry(item.clone())
        let count = map.entry(item.clone()).or_insert(0);

        // count 的类型是 &mut u32。
        // 这里修改的是 count 指向的值，所以不需要写 let mut count。
        // 只有当你要让 count 变量本身重新指向另一个引用时，才需要 let mut count。
        *count += 1;

        // 上面两行也可以合并成一行，这是 Rust 里统计频次的常见写法：
        // *map.entry(item.clone()).or_insert(0) += 1;
    });

    // 另一种写法：用 for 循环。
    // 初学时 for 循环通常更容易读；for_each 更偏函数式风格。
    //
    // let mut map = HashMap::new();
    // for item in items {
    //     *map.entry(item.clone()).or_insert(0) += 1;
    // }
    // map

    // 如果想减少 clone 次数，可以先 get_mut 查询，只有第一次插入时才 clone：
    //
    // let mut map = HashMap::new();
    // for item in items {
    //     if let Some(count) = map.get_mut(item.as_str()) {
    //         *count += 1;
    //     } else {
    //         map.insert(item.clone(), 1);
    //     }
    // }
    // map

    map
}

fn print_counts(counts: &HashMap<String, u32>) {
    counts.iter().for_each(|(item, count)| {
        println!("{}: {}", item, count);
    });

    // HashMap 常用方法小抄：
    //   counts.get("book")       -> 根据 key 查询，返回 Option<&u32>
    //   counts.contains_key("x") -> 判断 key 是否存在
    //   counts.keys()            -> 遍历所有 key
    //   counts.values()          -> 遍历所有 value
    //   counts.len()             -> 键值对数量
    //   counts.is_empty()        -> 是否为空
    //
    // 注意：HashMap 默认不保证遍历顺序，所以打印顺序可能每次不同。
    // 如果需要稳定顺序，可以把 keys 收集到 Vec 后排序，或者使用 BTreeMap。
}

fn main() {
    let items = build_items();

    // Vec 常用方法小抄：
    //   items.len()        -> 元素数量
    //   items.is_empty()   -> 是否为空
    //   items.get(0)       -> 安全获取第 0 个元素，返回 Option<&String>
    //   items.push(x)      -> 追加元素；要求 Vec 变量本身是 mut
    //   items.iter()       -> 借用遍历，元素类型是 &String
    //   items.into_iter()  -> 消费 Vec 遍历，元素类型是 String
    //
    // 本例后面还要使用 items，所以传给 count_items 时用 &items 借用它。
    let map = count_items(&items);

    print_counts(&map);

    // HashMap::len() 返回 key 的数量。
    // 这里不是商品总数 6，而是不同商品种类数，例如 book / pen / notebook 共 3 种。
    println!("total kinds: {}", map.len());
}

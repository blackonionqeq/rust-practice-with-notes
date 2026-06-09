// 参数 words 的类型是 &[&str]，可以从里往外拆：
//
// 1. &str
//    str 是一段 UTF-8 字符串内容，本身大小不固定，所以通常通过 &str 使用。
//    &str 不是只保存一个地址，而是一个“胖指针”，里面可以理解为：
//    - ptr: 指向字符串内容开头的地址
//    - len: 字符串内容的字节长度
//
// 2. &[T]
//    [T] 是一段连续的 T 元素，本身大小也不固定，所以通常通过 &[T] 使用。
//    &[T] 也是一个“胖指针”，里面可以理解为：
//    - ptr: 指向第一个 T 元素的地址
//    - len: 元素个数
//
// 3. &[&str]
//    把 T 具体替换成 &str，就是“指向一段连续 &str 元素的切片引用”。
//    这段切片里的每个元素又都是一个 &str 胖指针。
//    所以它的结构大致是：
//
//    words: &[&str]
//      ├─ ptr -> 第一个 &str 元素
//      └─ len -> 有几个 &str 元素
//
//    每个 &str 元素：
//      ├─ ptr -> 某段字符串内容
//      └─ len -> 这段字符串的字节长度
fn clean_words(words: &[&str]) -> Vec<String> {
    words
        .iter()
        // iter() 把切片变成迭代器。
        // 因为 words 里的元素类型是 &str，而 iter() 会按引用借用每个元素，
        // 所以这里每个 item 的类型是 &&str：它是“指向 &str 元素的引用”。
        // Iterator 是“惰性”的：map/filter 只是描述处理步骤，直到 collect/sum/count 等消费方法
        // 被调用时才真正执行。
        //
        // 常用的 Iterator 方法：
        // - map: 把每个元素转换成另一个值。
        // - filter: 只保留满足条件的元素。
        // - collect: 把迭代结果收集成 Vec、HashMap、String 等集合。
        // - sum/product: 对元素求和/求积。
        // - count: 统计元素个数。
        // - any/all: 判断是否存在任意元素满足条件/是否所有元素都满足条件。
        // - find/position: 找到第一个满足条件的元素/位置。
        // - enumerate: 给元素加上索引，得到 (index, item)。
        // - zip: 把两个迭代器按位置配对。
        // - take/skip: 只取前 N 个/跳过前 N 个。
        // - fold: 从一个初始值开始，把元素逐个累积成一个结果。
        .map(|item| item.trim().to_string().to_lowercase())
        // map 里先 trim 去掉首尾空白，再转成 String，最后转小写。
        // 这里 to_string() 后再 to_lowercase() 是可以的；也可以写成 item.trim().to_lowercase()。
        //
        // chars().count() 统计的是 Unicode 字符数量，不是字节数。
        // 中文、emoji 等字符用 len() 统计字节时容易和直觉不同。
        .filter(|item| item.chars().count() >= 3)
        // filter 只保留字符数 >= 3 的单词。
        //
        // collect 需要知道目标集合类型。这里用 turbofish 语法指定 Vec<String>：
        // collect::<Vec<String>>()。
        .collect::<Vec<String>>()
}

fn total_chars(words: &[String]) -> usize {
    // 这里参数写成 &[String]，而不是 &Vec<String>，是 Rust 里常见的接口写法：
    // “只需要读一段连续的 String 元素”时，切片引用比 Vec 引用更通用。
    //
    // main 里传的是 &words，其中 words 的类型是 Vec<String>。
    // &Vec<String> 能传给 &[String]，不是因为它们实现了某个“共同 trait”，
    // 而是因为 Vec<T> 实现了 Deref<Target = [T]>。
    // 编译器在需要 &[String] 的地方，可以做解引用强制转换：
    //
    //     &Vec<String>  ->  &[String]
    //
    // 内存结构可以这样理解：
    //
    // Vec<String>
    //   ├─ ptr      -> 堆上的第一个 String 元素
    //   ├─ len      -> 当前有几个 String
    //   └─ capacity -> 当前最多能放几个 String
    //
    // &[String]
    //   ├─ ptr      -> 第一个 String 元素
    //   └─ len      -> 有几个 String
    //
    // 所以 &[String] 可以看作是从 Vec<String> 借出来的一个“只关心 ptr + len 的视图”。
    words
        .into_iter()
        // 对切片 &[String] 调用 into_iter() 时，不会拿走 String 的所有权；
        // 它等价于按引用遍历，所以 word 的类型是 &String。
        .map(|word| word.chars().count())
        // sum 是消费方法：它会真正跑完整条迭代器管道，并把 usize 数值加起来。
        .sum()
}

fn main() {
    let words = clean_words(&[" WHAT", "the ", "hell!", "aa"]);
    println!("words: {:?}", words);
    println!("total chars: {}", total_chars(&words));
}

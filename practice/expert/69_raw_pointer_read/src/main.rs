fn first_byte(data: &[u8]) -> u8 {
    // 拿指针本身是safe的
    let p: *const u8 = data.as_ptr();
    // 解裸指针的引用是unsafe的
    // Safety: 确保指针指向有效的内存
    // 借用保证的是非空、对齐、指向有效数据、收生命周期约束
    // 裸指针 *const 没有这些保证，所以解引用时要声明unsafe和增加解释
    unsafe { *p }
}

fn main() {
    let data = vec![66, 2, 3];
    let result = first_byte(&data);
    println!("{result}");
    // 用空u8切片调用first_byte是unsafe的，会导致未定义行为
}

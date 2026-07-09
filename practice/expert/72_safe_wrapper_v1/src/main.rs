fn first_byte(data: &[u8]) -> Option<u8> {
    // 拿指针本身是safe的
    let p: *const u8 = data.as_ptr();
    // 前面证明p指向非空，则可以安全拿
    let result = unsafe { *p };
    Some(result)
}

fn main() {
    let data = vec![66, 2, 3];
    if let Some(result) = first_byte(&data) {
        println!("{result}");
    }
}

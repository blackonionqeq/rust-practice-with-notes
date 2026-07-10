use std::mem::offset_of;

#[repr(C)]
struct DemoC {
    a: u8,
    b: u64,
    c: u8,
}

unsafe extern "C" {
    fn add_bytes(a: u8, b: u8) -> u8;
}

struct DemoRust {
    a: u8,
    b: u64,
    c: u8,
}

fn main() {
    // 在常见 64 位目标上，C 布局保留字段顺序，DemoC 通常是 24 字节；
    // Rust 默认布局可以重排字段，DemoRust 可能是 16 字节，但这个观察结果不是稳定契约。
    println!("size_of DemoC: {}", std::mem::size_of::<DemoC>());
    println!("size_of DemoRust: {}", std::mem::size_of::<DemoRust>());
    println!("offset_of (DemoC, b): {}", offset_of!(DemoC, b));
    println!("offset_of (DemoC, c): {}", offset_of!(DemoC, c));
    println!("offset_of (DemoRust, b): {}", offset_of!(DemoRust, b));
    println!("offset_of (DemoRust, c): {}", offset_of!(DemoRust, c));
    // 若没有匹配的c实现链接上来，调用会报错
    // unsafe { add_bytes(1, 2) };
}

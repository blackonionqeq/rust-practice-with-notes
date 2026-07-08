/// # Safety
///
/// ptr 必须指向非空、指向合法的，已初始化值的 i32
/// 指向的值不能被起&mut i32类型的别名
unsafe fn read_i32(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

fn main() {
    let x = 234;
    // 指针来自真实i32类型的值所以合法
    // 注意：这里即使不声明as *const i32也可以运行，因为会发生自动类型转换
    let result = unsafe { read_i32(&x as *const i32) };
    println!("{result}");

    // 不正确用法（别调用）：违反前提1的空指针
    // let _ = unsafe { read_i32(std::ptr::null()) };
}

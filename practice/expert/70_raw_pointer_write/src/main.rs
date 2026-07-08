fn negate_first(data: &mut [i32]) {
    if data.is_empty() {
        return;
    }
    let ptr = data.as_mut_ptr();
    // Safety: 前面检查过data不为空，所以ptr一定能取到值
    unsafe {
        *ptr = -*ptr;
    }
}

fn main() {
    let mut list = vec![111, 2, 3, 4, 5];
    negate_first(&mut list);
    println!("{list:?}");
}

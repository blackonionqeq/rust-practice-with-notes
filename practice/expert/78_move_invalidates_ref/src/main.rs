struct Fragile {
    data: [i32; 3],
    ptr: *const i32,
}
impl Fragile {
    fn new(data: [i32; 3]) -> Self {
        Self {
            data,
            ptr: std::ptr::null(),
        }
    }
    // Safety: new后马上调，别在move后调用
    unsafe fn read_ptr(&self) -> i32 {
        unsafe { *self.ptr }
    }
}

fn main() {
    let mut frag = Fragile::new([1, 2, 3]);
    frag.ptr = &frag.data[0];
    // frag.ptr = frag.data.as_ptr();
    println!("address of data[0]: {:p}", &frag.data[0]);
    println!("value of ptr: {:p}", frag.ptr);

    let frag2 = frag;
    // [i32;3] 被copy了
    println!("address of frag2.data[0]: {:p}", &frag2.data[0]);
    // 但注意这里是旧地值
    println!("value of frag2.ptr: {:p}", frag2.ptr);

    // 不要调read_ptr
    // unsafe { println!("{}", frag2.read_ptr()) };
}

// --- Snippet A ---
static mut COUNTER: u32 = 0;
fn increment() {
    unsafe {
        COUNTER += 1; // capability: 访问\修改static mut
    }
}

// --- Snippet B ---
fn read_through_ptr() {
    let x: i32 = 42;
    let p: *const i32 = &x;
    unsafe {
        println!("{}", *p); // capability: 解引用裸指针
    }
}

// --- Snippet C ---
unsafe fn double(x: i32) -> i32 {
    x * 2
}
fn call_unsafe() {
    unsafe {
        double(5); // capability: 调用unsafe函数
    }
}

// --- Snippet D ---
union Bits {
    i: i32,
    b: [u8; 4],
}
fn read_union() {
    let v = Bits { i: 0x01020304 };
    unsafe {
        println!("{}", v.i); // capability: 解引用union字段
    }
}

// --- Snippet E ---
unsafe trait Trusted {}
unsafe impl Trusted for i32 {} // capability: 实现unsafe trait

fn my_deref_example() {
    let x: i32 = 42;
    let p: *const i32 = &x;
    unsafe {
        println!("{}", *p); // capability: 解引用裸指针
    }
}

fn main() {
    increment();
    read_through_ptr();
    call_unsafe();
    read_union();
    my_deref_example();
}

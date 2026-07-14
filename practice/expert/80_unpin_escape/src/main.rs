fn main() {
    let mut pinned = Box::pin(23443i32);
    // i32 是 Unpin 类型，无需 unsafe 可用 get_mut
    let ptr: &mut i32 = pinned.as_mut().get_mut();
    *ptr = 999;
    println!("{pinned}"); // 999

    fn requires_unpin<T: Unpin>(_: T) {}
    requires_unpin(pinned); // 编译通过

    // requires_unpin(Unmovable {}); // 编译失败
}

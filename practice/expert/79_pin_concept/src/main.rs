use std::marker::PhantomPinned;
use std::pin::Pin;

struct Unmovable {
    data: String,
    _pin: PhantomPinned,
}

fn main() {
    let pinned: Pin<Box<Unmovable>> = Box::pin(Unmovable {
        data: String::from("Hello aaa"),
        _pin: PhantomPinned,
    });
    println!("{}", pinned.data);

    // pin 里有 !Unpin（PhantomPinned）则不能被move
    // 下面这行会编译报错
    // let moved = *pinned;
}

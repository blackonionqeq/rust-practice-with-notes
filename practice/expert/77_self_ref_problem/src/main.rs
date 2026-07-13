// struct SelfRef<'a> {
//     data: String,
//     view: &'a str,
// }

// impl<'a> SelfRef<'a> {
//     fn new(data: String) -> Self {
//         Self {
//             data,
//             // data被move了，所以这里引用会报错
//             view: &data,
//         }
//     }
// }
//

struct TokenView {
    data: String,
    // 为避免自引用问题，可用外置index变量+view来获取切片
    start: usize,
    end: usize,
}

impl TokenView {
    fn new(data: String, start: usize, end: usize) -> Self {
        Self { data, start, end }
    }

    fn view(&self) -> &str {
        &self.data[self.start..self.end]
    }
}

fn main() {
    let data = String::from("hello, world");
    let view = TokenView::new(data, 0, 5);
    println!("{}", view.view());
}

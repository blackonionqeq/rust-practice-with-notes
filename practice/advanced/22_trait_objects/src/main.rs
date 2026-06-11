trait Render {
    fn render(&self) -> String;
}

struct Button {
    label: String,
}
struct Text {
    value: String,
}

impl Render for Button {
    fn render(&self) -> String {
        format!("[button:{}]", self.label)
    }
}

impl Render for Text {
    fn render(&self) -> String {
        format!("text:{}", self.value)
    }
}

fn render_all(items: &[Box<dyn Render>]) -> Vec<String> {
    // 这里借用的是 slice，不取得 Vec 的所有权；调用方之后仍然可以继续使用原集合。
    items.iter().map(|item| item.render()).collect()
}

fn main() {
    // Vec 里的每个元素必须有同一个、编译期已知的大小。
    // Text 和 Button 是不同具体类型，大小可能不同，不能直接混放进同一个 Vec。
    let mut should_render: Vec<Box<dyn Render>> = Vec::new();

    // Box<T> 会把具体值放到堆上，Vec 里只保存固定大小的 Box 指针。
    // dyn Render 表示“某个实现了 Render 的类型”，具体是 Button 还是 Text 在运行时决定。
    // Box<dyn Render> = 固定大小的指针 + 动态分发所需的信息，适合存放多种实现同一 trait 的值。
    should_render.push(Box::new(Button {
        label: "Click me".to_string(),
    }));
    should_render.push(Box::new(Text {
        value: "Hello, world!".to_string(),
    }));

    // 调用 item.render() 时会通过 trait object 动态分发到 Button 或 Text 各自的实现。
    let rendered = render_all(&should_render);
    for item in rendered {
        println!("{}", item);
    }
}

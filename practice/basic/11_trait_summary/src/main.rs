// trait 约束共同的特征，类似ts的抽象类
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Comment {
    username: String,
    content: String,
}
// 可为结构体实现特征
impl Summary for Article {
    fn summarize(&self) -> String {
        // format!宏 可以灵活地生成String
        format!("{} by {}", self.title, self.author)
    }
}
impl Summary for Comment {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let article = Article {
        title: String::from("Rust Notes"),
        author: String::from("black"),
    };
    let comment = Comment {
        username: String::from("reader"),
        content: String::from("nice post"),
    };
    println!("{}", article.summarize());
    println!("{}", comment.summarize());
    //    println!("Hello, world!");
}

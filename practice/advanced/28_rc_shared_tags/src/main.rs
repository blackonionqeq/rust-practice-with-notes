use std::rc::Rc;

#[derive(Debug)]
struct Article {
    title: String,
    tags: Vec<std::rc::Rc<String>>,
}

fn make_tag(name: &str) -> Rc<String> {
    // Rc<T> 表示“引用计数的共享所有权”，适合单线程里让多个值共享同一份数据。
    Rc::new(name.into())
}

fn make_article(title: &str, tags: Vec<Rc<String>>) -> Article {
    Article {
        title: title.into(),
        tags,
    }
}

fn tag_names(article: &Article) -> Vec<&str> {
    // 这里返回的是 &str，借用了 Rc<String> 里的字符串内容，没有克隆内部的 String。
    article.tags.iter().map(|tag| tag.as_str()).collect()
}

fn main() {
    let tag1 = make_tag("rust");
    let tag2 = make_tag("practice");
    println!(
        "before creating article1 Rc::strong_count: {}",
        Rc::strong_count(&tag1)
    );
    // Rc::clone(&tag1) 只增加引用计数，不会复制底层的 String。
    let article1 = make_article("title", vec![Rc::clone(&tag1)]);
    println!("{:?}", article1);
    println!(
        "after creating article1 Rc::strong_count: {}",
        Rc::strong_count(&tag1)
    );
    println!(
        "before creating article2 Rc::strong_count: {}",
        Rc::strong_count(&tag1)
    );
    // article1、article2 和 tag1 现在会共享同一个 "rust" 标签。
    let article2 = make_article("title", vec![Rc::clone(&tag1), Rc::clone(&tag2)]);
    println!("{:?}", article2);
    println!(
        "after creating article2 Rc::strong_count: {}",
        Rc::strong_count(&tag1)
    );

    println!("{} tag names:", article2.title);
    println!("{:?}", tag_names(&article2));
}

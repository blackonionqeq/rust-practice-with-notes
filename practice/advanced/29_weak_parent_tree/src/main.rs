use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String,
    // 为啥要加两层(RefCell + Weak)？因为解决不同问题，Weak是引用，RefCell让引用可变
    // 子节点的父引用用Weak而非强引用
    parent: RefCell<Weak<Node>>,
    // 父节点对子用强引用
    children: RefCell<Vec<Rc<Node>>>,
}

fn new_node(name: &str) -> Rc<Node> {
    Rc::new(Node {
        name: name.into(),
        parent: RefCell::new(Weak::new()),
        // 注意不用加rc：children: RefCell::new(vec![Rc::new()])
        children: RefCell::new(vec![]),
    })
}

fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    // Rc -> Weak use Rc::downgrade
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

fn parent_name(node: &Rc<Node>) -> Option<String> {
    //  weak -> rc use Weak::upgrade, and result type is Option<Rc<Node>>
    let res: Option<Rc<Node>> = node.parent.borrow().upgrade();
    if let Some(parent) = res {
        Some(parent.name.clone())
    } else {
        None
    }
    // Option能用map方法，等价于上面的写法
    // 顺便一提Result 也可以用map
    // res.map(|parent| parent.name.clone())
}

fn main() {
    let root = new_node("root");
    let leaf = new_node("leaf");
    add_child(&root, &leaf);
    println!("parent_name: {:?}", parent_name(&leaf));

    // strong_count 表示有多少个 Rc 正在强拥有 root；归零时 Node 会被自动释放。
    // weak_count 表示有多少个 Weak 指向 root；Weak 不阻止 Node 被释放。
    // 这里主要用于观察引用关系，确认 leaf.parent 没有形成强引用环。
    println!("Rc::strong_count: {}", Rc::strong_count(&root));
    println!("Rc::weak_count: {}", Rc::weak_count(&root))
}

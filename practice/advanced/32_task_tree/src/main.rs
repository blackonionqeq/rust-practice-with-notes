use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Task {
    title: String,
    done: RefCell<bool>,
    parent: RefCell<Weak<Task>>,
    children: RefCell<Vec<Rc<Task>>>,
}

fn new_task(title: &str) -> Rc<Task> {
    Rc::new(Task {
        title: title.to_string(),
        done: RefCell::new(false),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

fn add_child(parent: &Rc<Task>, child: &Rc<Task>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

fn mark_done(task: &Rc<Task>) {
    *task.done.borrow_mut() = true;
}

fn is_done(task: &Rc<Task>) -> bool {
    *task.done.borrow()
}

fn parent_title(task: &Rc<Task>) -> Option<String> {
    task.parent
        .borrow()
        .upgrade()
        .map(|parent| parent.title.to_string())
}

fn child_titles(task: &Rc<Task>) -> Vec<String> {
    task.children
        .borrow()
        .iter()
        .map(|child| child.title.to_string())
        .collect()
}

fn count_tasks(task: &Rc<Task>) -> usize {
    1 + task
        .children
        .borrow()
        .iter()
        .map(|child| count_tasks(child))
        .sum::<usize>()
}

fn main() {
    let task = new_task("Learn Rust");
    let children = ["Read notes", "Do exercises", "Review code"]
        .iter()
        .map(|title| new_task(title))
        .collect::<Vec<_>>();
    children.iter().for_each(|child| add_child(&task, child));
    mark_done(&children[1]);
    println!("Children titles: {:?}", child_titles(&task));
    println!(
        "Do exercises's parent title: {:?}",
        parent_title(&children[1])
    );
    println!("Do exercises is done?: {}", is_done(&children[1]));
    println!("total tasks: {}", count_tasks(&task));
    println!("Rc::strong_count: {}", Rc::strong_count(&task));
    // weak_count: 3 from children's parent
    println!("Rc::weak_count: {}", Rc::weak_count(&task));
}

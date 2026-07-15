// 方案一：用list + index 指；但不适用于list会变化（例如排序，删除，插入等场景）的情况
struct Task {
    name: String,
    parent: Option<usize>,
}

fn print_task_chain(tasks: &[Task], mut index: usize) {
    print!("{}", tasks[index].name);
    while let Some(parent) = tasks[index].parent {
        print!(" -> {}", tasks[parent].name);
        index = parent;
    }
}

use std::rc::Rc;


// 方案二：Rc + 引用
struct TaskNode {
    name: String,
    parent: Option<Rc<TaskNode>>,
}

fn print_task_chain_node(node: &TaskNode) {
    print!("{}", node.name);
    if let Some(parent) = &node.parent {
        print!(" -> ");
        print_task_chain_node(parent);
    }
}



fn main() {
    let base_tasks = ["Task 1", "Task 2", "Task 3"];
    let tasks = vec![
        Task {
            name: base_tasks[0].to_string(),
            parent: None,
        },
        Task {
            name: base_tasks[1].to_string(),
            parent: Some(0),
        },
        Task {
            name: base_tasks[2].to_string(),
            parent: Some(1),
        },
    ];
    print_task_chain(&tasks, 2);

    // let mut task_nodes = base_tasks.iter().map(|name| Rc::new(TaskNode {
    //     name: name.to_string(),
    //     parent: None,
    // })).collect::<Vec<Rc<TaskNode>>>();
    // 不能这样写，因为只有Rc没有RefCell，拿不到可变引用，就改写不了
    // task_nodes[1].as_mut().parent = Some(
    //     Rc::clone(&task_nodes[0])
    // );
    // 注意创建时就要被Rc包一层
    let task_node1 = Rc::new(TaskNode {
        name: base_tasks[0].into(),
        parent: None,
    });
    let task_node2 = Rc::new(TaskNode {
        name: base_tasks[1].into(),
        parent: Some(Rc::clone(&task_node1)),
    });
    let task_node3 = Rc::new(TaskNode {
        name: base_tasks[2].into(),
        parent: Some(Rc::clone(&task_node2)),
    });
    print_task_chain_node(&task_node3);


}

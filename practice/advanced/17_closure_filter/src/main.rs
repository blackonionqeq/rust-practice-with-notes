#[derive(Debug)]
struct Task {
    title: String,
    done: bool,
    priority: u8,
}

fn filter_tasks<F>(tasks: &[Task], mut predicate: F) -> Vec<&Task>
where
    F: FnMut(&Task) -> bool,
{
    // 这里不能直接写 `.filter(predicate)`，核心原因是引用层级不同。
    //
    // 1. `tasks` 的类型是 `&[Task]`，所以 `tasks.iter()` 迭代出来的元素是 `&Task`。
    // 2. `Iterator::filter` 调用判断函数时，会把“迭代出来的元素”再借用一次。
    //    因此传给 filter 闭包的参数类型实际是 `&&Task`。
    // 3. 但我们给 `filter_tasks` 设计的 predicate 类型是 `Fn(&Task) -> bool`，
    //    它期望接收的是 `&Task`，不是 `&&Task`。
    //
    // 所以这里用一个小闭包做“转接”：
    // - `task` 实际是 `&&Task`
    // - 调用 `predicate(task)` 时，Rust 会自动解引用，把它调整成 `&Task`
    //
    // 更显式的写法是：`.filter(|task| predicate(*task))`
    // 其中 `*task` 会把 `&&Task` 变成 `&Task`。
    //
    // `filter` 自己接收的是 `FnMut`，不是更严格的 `Fn`：
    // - `FnOnce`：至少能调用一次，可能会消耗捕获的值。
    // - `FnMut`：能调用多次，并且允许修改自己捕获的状态。
    // - `Fn`：能调用多次，但不能修改自己捕获的状态。
    //
    // iterator 方法通常会逐个处理元素，需要反复调用闭包；
    // 同时它们一般允许闭包在调用之间记录状态，比如计数、累加、去重等。
    // 所以 `.map`、`.filter`、`.find`、`.any`、`.all`、`.for_each`、
    // `.position`、`.take_while`、`.skip_while`、`.inspect`、`.fold`
    // 这些常见方法的闭包参数大多是 `FnMut`。
    //
    // 因此我们自己的 `filter_tasks` 也用 `FnMut(&Task) -> bool`，
    // 这样既能接收普通不改状态的闭包，也能接收会更新捕获状态的闭包。
    tasks.iter().filter(|task| predicate(*task)).collect()
}

fn main() {
    const MIN_PRIORITY: u8 = 3;
    let tasks = vec![
        Task {
            title: "Task 1".to_string(),
            done: false,
            priority: 1,
        },
        Task {
            title: "Task 2".to_string(),
            done: true,
            priority: 6,
        },
        Task {
            title: "Task 3".to_string(),
            done: false,
            priority: 8,
        },
        Task {
            title: "Task 4".to_string(),
            done: true,
            priority: 2,
        },
    ];

    let filtered_tasks = filter_tasks(&tasks, |task| task.priority >= MIN_PRIORITY);
    println!("{:?}", &filtered_tasks);
    let filtered_tasks = filter_tasks(&tasks, |task| task.done);
    println!("{:?}", &filtered_tasks);
}

#[derive(Debug)]
enum List {
    // Box<T> 用于固定大小，只取地址，所以大小能确定
    // 拥有所有权，离开作用域后值会被释放
    Cons(i32, Box<List>),
    Nil,
}

fn prepend(list: List, value: i32) -> List {
    List::Cons(value, Box::new(list))
}

// 注意len, sum, to_vec 都是用的不可变借用
fn len(list: &List) -> usize {
    match list {
        List::Cons(_, next) => 1 + len(next),
        List::Nil => 0,
    }
}

fn sum(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => *value + sum(next),
        List::Nil => 0,
    }
}

fn to_vec(list: &List) -> Vec<i32> {
    match list {
        List::Cons(value, next) => {
            // 自递归
            let mut vec = to_vec(next);
            vec.insert(0, *value);
            vec
        }
        List::Nil => vec![],
    }
}

fn main() {
    let mut list = List::Nil;
    // for num in [3, 2, 1] {
    //     list = prepend(list, num);
    // }
    list = [3, 2, 1].iter().fold(list, |list, cur| prepend(list, *cur));
    println!("{:?}", list);
    println!("len: {}", len(&list));
    println!("sum: {}", sum(&list));
    println!("{:?}", to_vec(&list));
}

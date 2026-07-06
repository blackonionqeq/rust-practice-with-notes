struct Point {
    x: f64,
    y: f64,
}

// 不用宏，可自定义格式
// 另外宏没有运行时开销，会在编译器展开
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

// 实现==语义
// 部分eq是因为部分无法自相等（NaN != NaN）
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 2.0, y: 3.0 };
    println!("{p1:?}");
    println!("{p2:?}");
    println!("is eq: {}", p1 == p2);
}

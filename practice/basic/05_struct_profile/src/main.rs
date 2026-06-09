#[derive(Debug)]
struct UserProfile {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> UserProfile {
    // 单内容返回，不用加元祖（的括号）
    UserProfile {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    //println!("Hello, world!");
    let user = build_user(String::from("black"), String::from("black@example.com"));
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    let user2 = UserProfile {
        email: String::from("second@example.com"),
        // struct update syntax
        // 注意user的所有权被转移走了，不能再使用user
        ..user
    };
    // {:?}打印Debug impl的结构体
    println!("user2: {:?}", user2);
}

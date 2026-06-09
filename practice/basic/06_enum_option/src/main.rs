#[derive(Debug)]
enum LoginStatus {
    Guest,
    User(String),
}

fn display_status(status: LoginStatus) {
    match status {
        // 枚举必须带枚举名+:: LoginStatus::
        LoginStatus::Guest => {
            println!("guest user")
        }
        LoginStatus::User(name) => {
            println!("logged in as {}", name)
        }
    }
}

fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("black")),
        2 => Some(String::from("rustacean")),
        _ => None,
    }
}

fn main() {
    display_status(LoginStatus::Guest);
    display_status(LoginStatus::User(String::from("black")));
    match find_user(1) {
        // 注意这里是用user 匹配了String类型，所以下一行可直接打印
        Some(user) => {
            println!("found user: {}", user)
        }
        None => {
            println!("user not found")
        }
    }
    match find_user(99) {
        Some(user) => {
            println!("found user: {}", user)
        }
        None => {
            println!("user not found")
        }
    }
    //println!("Hello, world!");
}

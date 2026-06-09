mod shop {
    // 声明pub 会让全局可用
    pub mod inventory {
        pub fn print_item(name: &str) {
            println!("item: {}", name)
        }
        pub fn item_count() -> u32 {
            3
        }
    }
}
fn print_shop_status() {
    let count = shop::inventory::item_count();
    println!("count: {}", count);
    shop::inventory::print_item("book");
}
fn main() {
    print_shop_status();
    shop::inventory::print_item("pen");
}

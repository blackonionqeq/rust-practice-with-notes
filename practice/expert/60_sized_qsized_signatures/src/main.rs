/// 这题的主题思想是把不同类型统一成某种视图，再对这种视图取长度
fn byte_len<T: ?Sized + AsRef<[u8]>>(value: &T) -> usize {
    // 注意value是&(?Sized + AsRef<[u8]>), as_ref()返回的是&[u8]，这种胖指针能调用len取长度
    // 注意是对&[u8]取长度，不是[u8]
    value.as_ref().len()
}
/**
 * 类似这样：
 *
 trait MyLen {
     fn my_len(&self) -> usize;
 }

 impl MyLen for str {
     fn my_len(&self) -> usize {
         self.len()
     }
 }

 impl MyLen for [u8] {
     fn my_len(&self) -> usize {
         self.len()
     }
 }

 impl MyLen for String {
     fn my_len(&self) -> usize {
         self.len()
     }
 }

 impl MyLen for Vec<u8> {
     fn my_len(&self) -> usize {
         self.len()
     }
 }
 fn get_len<T: ?Sized + MyLen>(value: &T) -> usize {
     value.my_len()
 }
 */

fn main() {
    println!(
        "get byte_len by String: {}",
        byte_len(&String::from("hello, this is string"))
    );

    println!("get byte_len by &str: {}", byte_len("hello, this is &str"));

    println!("get byte_len by Vec<u8>: {}", byte_len(&vec![1, 2, 3]));

    println!(
        "get byte_len by &[u8]: {}",
        byte_len("hello, this is string literal".as_bytes())
    );
}

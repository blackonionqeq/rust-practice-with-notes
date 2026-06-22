use std::time::Duration;

// 想体验http库可cargo add reqwest，然后取消注释
// 尝试完后，cargo remove reqwest，cargo clean清掉缓存（reqwest很大，取消注释后run一次，文件夹约900MB）
// use reqwest;
// 用time，所以要加上features里的time
use tokio::{time::sleep};
// use tokio::{macros, runtime};

// async fn fetch_data() -> String {
//     let result = reqwest::get("https://www.example.com")
//         .await
//         .unwrap()
//         .text()
//         .await
//         .unwrap();
//     result
// }

async fn fetch_label(id: u32) -> String {
    // 注意这里用的是tokio的sleep，不是std::time::sleep的。后者会阻塞当前线程
    sleep(Duration::from_millis(500)).await;
    format!("label-{id}")
}

// 要加上这个，才能让main前面带上async
// 这行等价于 #[tokio::main(flavor = "multi_thread")]，所以features里要加上rt-multi-thread
#[tokio::main]
async fn main() {
    // let future = fetch_data();
    let result = fetch_label(42).await;
    println!("{result}");
}

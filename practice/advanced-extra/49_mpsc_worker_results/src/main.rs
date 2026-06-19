use std::env;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let parts = env::args().nth(1).unwrap_or("4".into());
    let parts: usize = parts.parse().unwrap_or(4);

    let list = (0..200).collect::<Vec<_>>();
    let chunk_size = list.len().div_ceil(parts);
    let chunks = list
        .chunks(chunk_size)
        // 让每个chunk变成一个vec
        .map(|chunk| chunk.to_vec())
        // 把外层也变成vec
        .collect::<Vec<_>>();

    for chunk in chunks {
        // sender 可以 clone，但 receiver 不能
        let sender = sender.clone();
        thread::spawn(move || {
            let result = chunk.iter().sum::<i32>();
            // println!("current result: {result}");
            // 记得match，否则发送失败时会panic
            // receiver 被drop的话，会失败
            match sender.send(result) {
                Ok(_) => {}
                Err(e) => eprintln!("{e}"),
            }
        });
    }

    // 原始sender需要手动drop
    drop(sender);

    // 所有sender 都被drop，且channel没数据时，recv()会失败
    let mut sum = 0;
    while let Some(result) = receiver.recv().ok() {
        sum += result;
    }
    // 或者直接这样
    // let sum = receiver.iter().sum::<i32>();

    println!("sum: {sum}");
}

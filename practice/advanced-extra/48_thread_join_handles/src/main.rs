use std::env;
use std::thread;

fn main() {
    let parts = env::args().nth(1).unwrap_or("3".to_string());
    let num_threads = parts.parse::<usize>().unwrap_or(3).max(1);
    // range 通过 collect 转换成 Vec<usize>
    let list = (1..100).collect::<Vec<usize>>();
    let chunk_size = list.len().div_ceil(num_threads);

    // thread::spawn 要求闭包捕获的数据能在线程中独立存活。
    // list.chunks(...) 产生的是 &[usize]，直接把切片借用传进线程会借用 list，
    // 不满足线程需要的 'static 生命周期；所以这里把每个 chunk 拷贝成 owned Vec。
    let chunks = list
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<_>>();

    println!("chunks: {chunks:?}");

    let handles = chunks
        .into_iter()
        .map(|chunk| {
            thread::spawn(move || {
                // 每个线程拿到自己的 Vec 所有权。
                // 不要在多个线程闭包里捕获同一个 slices Vec，否则会变成“同一个值被 move 多次”。
                let current_chunk = chunk;
                current_chunk.iter().sum::<usize>()
            })
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for handle in handles {
        // join 会让主线程等待这个子线程结束，并拿回子线程闭包的返回值。
        // 如果子线程 panic，这里会得到 Err，所以不要直接 unwrap。
        if let Ok(result) = handle.join() {
            total += result;
        }
    }

    println!("total: {total}");
}

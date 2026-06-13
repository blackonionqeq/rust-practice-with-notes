use std::env;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(
        env::args()
            .nth(1)
            .expect("usage: cli_args_pathbuf <input-file>"),
    );

    let text = std::fs::read_to_string(&path).unwrap_or_default();

    // display()返回可打印内容
    println!("file path: {}", path.display());
    // 取字符串"大小"的速记 ─ 名字喊啥就是啥:
    //   text.len()            -> 字节  裸 len: Rust 里字符串默认长度就是字节(O(1))
    //   text.as_bytes().len() -> 字节  名字喊了 bytes, 跟 len() 等价
    //   text.chars().count()  -> 字符  名字喊了 chars 才数字符; 要遍历解码去"数"(O(n))
    // 反直觉: 没有直接给"字符数"的 len; 想要字符数必须 chars().count()。
    // 换算: 多字节字符(中文/emoji)下三值不同, "你好"=2字符=6字节;
    //       chars() 数的是码点, 不是人眼字形(组合 emoji 会偏大, 要 unicode-segmentation)。
    println!(" {} bytes", text.as_bytes().len()); // 想要字节数, 也可直接写 text.len()
}

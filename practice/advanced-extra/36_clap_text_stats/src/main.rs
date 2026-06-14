// 先 cargo add clap --features derive
// cargo add clap 类似 npm i 依赖包名
// crate 会把一些能力做成可选功能，默认不启用
// clap 的 derive 特性可以让我们使用结构体来定义命令行参数，需要用--features derive 开启
//
// cargo 的 依赖管理强依赖SemVer，整体可类比npm
// 不过语法上有些区别："1.2.3" 这种默认写法，等价于 >=1.2.3 && < 2.0.0，对应npm里的 ^1.2.3
//
//
use clap::Parser;
use std::path::PathBuf;

// 这部分的说明看 notes/36_clap_text_stats.md
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, value_name = "PATH")]
    input: PathBuf,

    #[arg(long, default_value_t = 2)]
    min_count: usize,

    #[arg(long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();
    // 这是1.58.0 后的新写法，对应老写法的 ("{:?}", args)
    println!("{args:?}");
}

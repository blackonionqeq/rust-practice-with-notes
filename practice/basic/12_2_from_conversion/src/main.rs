struct Age {
    years: u32,
}
impl From<u32> for Age {
    // From trait 的核心定义可以理解成：
    //
    //     pub trait From<T> {
    //         fn from(value: T) -> Self;
    //     }
    //
    // 想确认定义时，可以去这几个地方找：
    // - 官方文档：std::convert::From
    // - 本地文档：`rustup doc --std` 后搜索 `From`
    // - IDE 里对 `From` 使用 “Go to Definition”
    //
    // 所以在 `impl From<u32> for Age` 里：
    // - `From<u32>` 里的 `u32` 是 from 的参数类型
    // - `for Age` 里的 `Age` 是目标类型，也就是返回类型
    // - `Self` 在这里就等于 `Age`
    //
    // 换句话说，这里的 from 应该接收一个 u32，并返回一个 Age。
    // 参数类型这里不能省略；虽然编译器已经知道 `From<u32>` 里的 `T` 是 `u32`，
    // 但实现 trait 方法时，方法签名还是必须完整写出来。
    //
    // 实现完 `From<u32> for Age` 之后，常见用法有两个：
    // - 作为关联函数调用：`Age::from(18)`
    // - 通过 `Into` 调用：`let age: Age = 18u32.into();`
    //
    // 第二种之所以可行，是因为标准库会基于 `From<T> for U`
    // 自动提供对应的 `Into<U> for T`。
    fn from(t: u32) -> Self {
        Age { years: t }
    }
}

fn describe_age(age: Age) -> String {
    format!("age: {}", age.years)
}
fn main() {
    let age1 = Age::from(18);
    let age2 = 20u32.into();
    println!("{}, {}", describe_age(age1), describe_age(age2))
}

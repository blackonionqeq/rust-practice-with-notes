use std::fmt::{Display, Formatter};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum AppError {
    Io(IoError),
    EmptyInput,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // REVIEW: 题目要求固定输出 `failed to read input`，
            // 这里现在带了原始 IO error，大小写也不一致。
            AppError::Io(e) => write!(f, "Failed to read input: {}", e),
            // REVIEW: 题目要求固定输出 `input is empty`，这里大小写不一致。
            AppError::EmptyInput => write!(f, "Input is empty"),
        }
    }
}

// FIXME: 这里还缺 `impl From<std::io::Error> for AppError`。
// 有了它以后，main 或其他函数里才能用 `?` 自动把 IO error 转成 AppError::Io。
// 修正后如下
impl From<IoError> for AppError {
    fn from(e: IoError) -> Self {
        AppError::Io(e)
    }
}

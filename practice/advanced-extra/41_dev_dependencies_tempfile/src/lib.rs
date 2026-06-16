pub fn count_lines(file: &str) -> usize {
    file.lines().count()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use anyhow::Result;
    use tempfile::NamedTempFile;
    // 要带上这个，不然writeln!宏会报错
    use std::io::Write;

    use super::*;

    // fn create_temp_file(content: &str) -> Result<PathBuf> {
    //     let mut tmp = NamedTempFile::new()?;
    //     writeln!(tmp, "{content}")?;
    //     // test里的除了错误不会输出到控制台，所以println!不会输出
    //     // println!("tmp: {:?}", read_to_string(tmp.path()));
    //     Ok(tmp.path().to_path_buf())
    // }

    fn test_count_lines_helper(content: &str, count: usize) -> Result<()> {
        let mut tmp = NamedTempFile::new()?;
        // 别用writeln，会多一行
        // 这里用了{}会被转义，想保留语义，要加反斜杠：\{\}
        // 或者用原始字符串字面量写法：r#"{"port":8080}"#
        write!(tmp, "{content}")?;
        let text = read_to_string(tmp.path())?;

        assert_eq!(count_lines(&text), count);
        Ok(())
    }

    #[test]
    fn test_count_lines() -> Result<()> {
        let tests: [(&str, usize); 3] = [("", 0), ("hello\n", 1), ("hello\nworld\n", 2)];
        for (content, count) in tests {
            test_count_lines_helper(content, count)?;
        }
        Ok(())
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// 这里的创建方式是： cargo new crate/text_core --lib --name text_core
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn text_count_works() {
        let result = text_count("Hello, world!");
        assert_eq!(result, 2);
    }
}

pub fn text_count(text: &str) -> usize {
    let mut count = 0;
    for line in text.lines() {
        count += line.split_whitespace().collect::<Vec<_>>().len();
    }
    count
}

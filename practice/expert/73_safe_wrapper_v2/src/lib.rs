pub fn read_at(data: &[i32], index: usize) -> Option<i32> {
    if data.is_empty() || index >= data.len() {
        return None;
    }
    let ptr = data.as_ptr();
    // Safety:
    // 1. index不为空（前面已证明）
    // 2. 指针偏移量不会超过切片长度（前面已证明）
    let value = unsafe { *ptr.add(index) };
    Some(value)
}

// 通过单元测试来测各种情况和边界
// 防止不小心改错内容，但不知道破坏了边界的情况
#[cfg(test)]
mod tests {
    use super::read_at;

    #[test]
    fn reads_valid_index() {
        assert_eq!(read_at(&[1, 444, 55, 777], 1), Some(444));
    }
    #[test]
    fn out_of_bounds() {
        assert_eq!(read_at(&[0], 5), None);
    }
    #[test]
    fn empty_slice() {
        assert_eq!(read_at(&[], 0), None);
    }
}

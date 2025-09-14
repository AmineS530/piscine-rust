pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (idx, num) in array.iter().rev().enumerate() {
        if *num == key {
            return Some(array.len() - 1 + idx);
        }
    }
    None
}

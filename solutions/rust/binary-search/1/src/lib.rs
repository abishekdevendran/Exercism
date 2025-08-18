pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let (mut l, mut r) = (0, array.len() - 1);
    while l <= r {
        let mid = (l + r) / 2;
        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => l = mid + 1,
            std::cmp::Ordering::Greater => {
                if mid == 0 { break; }
                r = mid - 1;
            }
        }
    }
    println!("{}, {}", l, r);
    None
}

pub fn series(digits: &str, len: usize) -> Vec<String> {
    // todo!("What are the series of length {len} in string {digits:?}")
    if len > digits.len() {
        return vec![];
    }
    (0..digits.len() + 1 - len)
        .map(|i| digits[i..i + len].to_string())
        .collect()
}

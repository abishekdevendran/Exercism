pub fn series(digits: &str, len: usize) -> Vec<String> {
    // todo!("What are the series of length {len} in string {digits:?}")
    digits
    .chars()
    .collect::<Vec<char>>()
    .windows(len)
    .map(|window| window.iter().collect::<String>())
    .collect()
}

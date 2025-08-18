pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|&el| {
            if primes.iter().all(|&p| el % p != 0) {
                primes.push(el);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}

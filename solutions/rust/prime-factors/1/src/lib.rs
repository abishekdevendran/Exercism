pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    (2..n+1).fold(Vec::new(), |mut acc, el|{
        while n % el == 0 {
            n /= el;
            acc.push(el);
        }
        acc
    })
}

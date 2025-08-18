pub fn factors(n: u64) -> Vec<u64> {
    if n==1 {
        return vec![]
    }
    let mut n = n;
    let mut temp=(2..((n as f64).sqrt().ceil() as u64)+1).fold(Vec::new(), |mut acc, el|{
        while n % el == 0 {
            n /= el;
            acc.push(el);
        }
        acc
    });
    if n>1 {
        temp.push(n);
    }
    temp
}

pub fn collatz(n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut ans = 0;
    let mut n = n;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        ans += 1;
    }
    match n {
        1 => Some(ans),
        _ => None,
    }
}

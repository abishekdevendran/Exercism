use std::cmp::max;

pub fn list_all_primes(n: u64) -> Vec<u64> {
    let max_limit = max(32, n * ((n as f64).ln()+(n as f64).ln().ln())as u64);
    println!("max_limit: {}", max_limit);
    let mut seen = vec![false; max_limit as usize];
    (2..max_limit)
        .filter(|&i| {
            if !seen[i as usize] {
                let mut temp = i * i;
                while temp < max_limit {
                    seen[temp as usize] = true;
                    temp += i;
                }
                true
            } else {
                false
            }
        })
        .collect()
}

pub fn nth(n: u32) -> u32 {
    list_all_primes((n+1) as u64)[n as usize] as u32
}

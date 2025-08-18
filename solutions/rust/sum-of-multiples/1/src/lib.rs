use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .fold(HashSet::new(), |mut acc: HashSet<u32>, el| {
            let mut temp = *el;
            // println!("Temp is {}.", temp);
            while temp < limit {
                acc.insert(temp);
                temp += el;
            }
            // println!("DEBUG: {:?}", acc);
            acc
        })
        .iter()
        .inspect(|&&x| println!("{}", x))
        .sum::<u32>()
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits: Vec<u32> = match code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect()
    {
        Some(ds) => ds,
        None => return false, // Contains a non-digit character (other than whitespace)
    };

    if digits.len() <= 1 {
        return false;
    }

    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d } else { d * 2 })
        .map(|d| if d > 9 { d - 9 } else { d })
        .sum::<u32>()
        % 10
        == 0
}

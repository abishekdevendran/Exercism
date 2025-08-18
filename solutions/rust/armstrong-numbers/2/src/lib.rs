pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let n = num.checked_ilog10().unwrap() + 1;
    num.to_string()
        .chars()
        .try_fold(0, |mut acc, el| {
            acc += el.to_digit(10).unwrap().pow(n);
            if acc > num {
                return Err(());
            }
            Ok(acc)
        })
        .is_ok_and(|el| num == el)
}

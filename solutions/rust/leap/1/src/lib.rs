pub fn is_leap_year(year: u64) -> bool {
    match year % 400 {
        0 => true,
        _ => match year % 100 {
            0 => false,
            _ => matches!(year % 4, 0),
        },
    }
}

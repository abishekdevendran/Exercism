#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_subset(main: &[i32], sub: &[i32]) -> bool {
    if sub.is_empty() {
        return true;
    }
    if sub.len() > main.len() {
        return false;
    }
    main.windows(sub.len()).any(|el| el == sub)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_subset(second_list, first_list) {
        return Comparison::Sublist;
    }
    if is_subset(first_list, second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty(){
        return String::new();
    }
    // todo!("build a proverb from this list of items: {list:?}")
    list.iter()
        .enumerate()
        .fold(String::new(), |acc, (idx, el)| {
            if idx == 0 {
                acc
            } else {
                acc + &format!("For want of a {} the {} was lost.\n", list[idx - 1], el)
            }
        }) + &format!("And all for the want of a {}.", list[0])
}

use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    // todo!("Check if the string \"{string}\" contains balanced brackets");
    let mut st = VecDeque::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => st.push_back(c),
            ')' => {
                if st.pop_back() != Some('(') {
                    return false;
                }
            },
            ']' => {
                if st.pop_back() != Some('[') {
                    return false;
                }
            },
            '}' => {
                if st.pop_back() != Some('{') {
                    return false;
                }
            },
            _ => (),
        }
    }
    st.is_empty()
}

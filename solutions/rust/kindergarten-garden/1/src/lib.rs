pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    fn char_to_plant(c: char) -> &'static str {
        match c {
            'V' => "violets",
            'R' => "radishes",
            'C' => "clover",
            'G' => "grass",
            _ => unreachable!(),
        }
    }
    fn student_to_idx(student: &str) -> usize {
        student.chars().next().unwrap() as usize - 'A' as usize
    }
    let idx = student_to_idx(student);
    let offset = (diagram.len() / 2) + 1;
    vec![
        char_to_plant(diagram.as_bytes()[2*idx] as char),
        char_to_plant(diagram.as_bytes()[2*idx + 1] as char),
        char_to_plant(diagram.as_bytes()[offset + 2*idx] as char),
        char_to_plant(diagram.as_bytes()[offset + 2*idx + 1] as char),
    ]
}

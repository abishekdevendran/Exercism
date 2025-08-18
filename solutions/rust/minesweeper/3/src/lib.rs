static DIRS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    (0..height)
        .map(|i| {
            let width = minefield[i as usize].len() as i32;
            (0..width)
                .map(|j| {
                    if minefield[i as usize].as_bytes()[j as usize] == b'*' {
                        return '*';
                    }
                    match DIRS
                        .iter()
                        .map(|(x, y)| (x + i, y + j))
                        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < height && *y < width)
                        .filter(|(x, y)| minefield[*x as usize].as_bytes()[*y as usize] == b'*')
                        .count()
                    {
                        0 => ' ',
                        n => (n as u8 + b'0') as char,
                    }
                })
                .collect()
        })
        .collect()
}

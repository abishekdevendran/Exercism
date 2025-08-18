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
    let (height, width) = (minefield.len() as i32, minefield[0].len() as i32);
    (0..height)
        .map(|i| {
            (0..width)
                .map(|j| {
                    if minefield[i as usize].as_bytes()[j as usize] == b'*' {
                        return '*';
                    }
                    println!(
                        "{:?}",
                        DIRS.iter()
                            .map(|(x, y)| { (x + i, y + j) })
                            .filter(|(x, y)| { *x >= 0 && *y >= 0 && *x < height && *y < width })
                    );
                    return ' ';
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

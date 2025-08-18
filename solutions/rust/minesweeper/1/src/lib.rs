pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() || minefield[0].is_empty() {
        return minefield.iter().map(|e| e.to_string()).collect();
    }
    let dirs: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut ans:Vec<Vec<u8>> = vec![];
    for i in 0..minefield.len() {
        let mut new_row: Vec<u8> = vec![];
        for j in 0..minefield[0].len() {
            if minefield[i].as_bytes()[j] == b'*' {
                new_row.push(b'*');
                continue;
            }
            let mut count = 0;
            for (x, y) in dirs {
                let (netx, nety) = (i as isize + x, j as isize + y);
                if netx >= 0
                    && nety >= 0
                    && netx < minefield.len() as isize
                    && nety < minefield[0].len() as isize
                    && minefield[netx as usize].as_bytes()[nety as usize] == b'*'
                {
                    count += 1;
                }
            }
            if count > 0 {
                new_row.push(count + b'0');
            } else {
                new_row.push(b' ');
            }
        }
        ans.push(new_row);
    }
    ans.iter().map(|e| String::from_utf8(e.to_vec()).unwrap()).collect()
}

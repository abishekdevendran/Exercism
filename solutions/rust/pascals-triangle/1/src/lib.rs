pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: (0..row_count).fold(vec![], |mut acc, row_idx| {
                let current_row_elements = (row_idx + 1) as usize;
                let mut new_row = Vec::with_capacity(current_row_elements);

                if row_idx == 0 {
                    // First row is always [1]
                    new_row.push(1);
                } else {
                    let prev_row = acc.last().expect("Previous row should exist if row_idx > 0");
                    new_row.push(1);
                    for i in 0..(prev_row.len() - 1) {
                        new_row.push(prev_row[i] + prev_row[i + 1]);
                    }

                    // Last element of any row (after the first) is 1
                    new_row.push(1);
                }
                acc.push(new_row);
                acc
            }),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

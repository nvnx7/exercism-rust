pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        if row_count > 0 {
            rows.push(vec![1])
        }

        for row_i in 1..row_count {
            let mut row: Vec<u32> = Vec::new();
            let prev_row = &rows[(row_i - 1) as usize];
            for i in 0..=row_i {
                let prev_n = if let Some(j) = (i as usize).checked_sub(1) {
                    *prev_row.get(j).unwrap()
                } else {
                    0
                };
                let next_n = *prev_row.get(i as usize).unwrap_or(&0);
                row.push(prev_n + next_n);
            }
            rows.push(row);
        }
        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}

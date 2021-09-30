pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let n_rows = input.len();
    for (i_row, row) in input.iter().enumerate() {
        for (i_col, n) in row.iter().enumerate() {
            if row.iter().all(|x| x <= n) && (0..n_rows).all(|x| input[x][i_col] >= *n) {
                points.push((i_row, i_col))
            }
        }
    }
    points
}

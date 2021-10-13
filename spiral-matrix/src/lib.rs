pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    let mut i_start: usize = 0;
    let mut i_end: usize = size as usize;
    let mut n: u32 = 1;

    while i_start <= (i_end.checked_sub(1).unwrap_or(0)) {
        for i in i_start..i_end {
            matrix[i_start][i] = n;
            n += 1;
        }
        for i in (i_start + 1)..i_end {
            matrix[i][i_end - 1] = n;
            n += 1;
        }
        for i in (i_start..(i_end.checked_sub(1).unwrap_or(0))).rev() {
            matrix[i_end - 1][i] = n;
            n += 1;
        }
        for i in ((i_start + 1)..(i_end.checked_sub(1).unwrap_or(0))).rev() {
            matrix[i][i_start] = n;
            n += 1;
        }
        i_start += 1;
        i_end = i_end.checked_sub(1).unwrap_or(0);
    }

    matrix
}

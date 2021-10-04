pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut range = (2..=upper_bound).collect::<Vec<u64>>();

    for i in 0..range.len() {
        let n = range[i];

        if n == 0 {
            continue;
        }
        for j in (0..range.len()).skip(i).step_by(n as usize) {
            if range[j] != n {
                range[j] = 0;
            }
        }
    }
    range
        .iter()
        .filter(|n| **n != 0)
        .copied()
        .collect::<Vec<u64>>()
}

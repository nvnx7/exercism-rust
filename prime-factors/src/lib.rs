pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut num: u64 = n;
    let mut divisor: u64 = 2;
    while num != 1 {
        if num % divisor == 0 {
            num = num / divisor;
            prime_factors.push(divisor);
        } else {
            divisor += 1;
        }
    }
    prime_factors
}

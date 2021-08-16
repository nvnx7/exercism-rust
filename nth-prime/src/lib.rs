pub fn nth(n: u32) -> u32 {
    let n1: usize = n as usize;

    let mut primes: Vec<u32> = vec![2];
    let mut num: u32 = 3;
    let mut is_prime: bool;
    while primes.len() <= n1 {
        is_prime = true;
        for p in primes.iter() {
            if num % p == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(num)
        }

        num += 2;
    }

    return *primes.last().unwrap();
}

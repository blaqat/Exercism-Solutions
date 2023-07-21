pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();

    let mut n = n;
    let mut c = 2;

    while c <= n {
        if n % c == 0 {
            prime_factors.push(c);
            n /= c;
        } else {
            c += 1;
        }
    }

    prime_factors
}

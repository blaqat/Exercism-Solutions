// Memozation version (Probably should use a struct)

static mut PRIMES: Vec<u32> = Vec::new();

fn is_prime(n: &u32) -> bool {
    unsafe {
        !PRIMES
            .iter()
            .take_while(|&p| p.pow(2) <= *n)
            .any(|p| n % p == 0)
    }
}

pub fn nth(n: u32) -> u32 {
    unsafe {
        while PRIMES.len() <= n as usize {
            PRIMES.push(
                (PRIMES.last().unwrap_or(&1) + 1..u32::MAX)
                    .find(is_prime)
                    .unwrap(),
            );
        }

        PRIMES[n as usize]
    }
}

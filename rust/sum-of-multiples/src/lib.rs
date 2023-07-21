pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|l| factors.iter().any(|&f| l.checked_rem(f) == Some(0)))
        .sum()
}

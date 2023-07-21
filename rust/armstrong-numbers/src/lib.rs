// Refactored, No need for vector anymore

pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = (num as f32).log10() as u32 + 1;

    (1..=num_digits)
        // Convert range into (digits from num) ^ num_digits
        .map(|pow| (num / 10_u32.pow(num_digits - pow) % 10).pow(num_digits))
        // Reduce iterator into sum of digits
        .try_fold(0_u32, |sum, digit| sum.checked_add(digit))
        // If add stayed in bounds, return if sum of digits equals num
        .is_some_and(|sum| sum == num)
}

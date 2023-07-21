/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // unimplemented!("Is the Luhn checksum for {code} valid?");
    let code: String = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    if code.len() < 2 || code.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    let double_num = |mut num: i32| -> i32 {
        num *= 2;
        if num > 9 {
            num -= 9;
        }
        num
    };

    code.as_bytes()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &byte)| (i, byte as i32 - 48))
        .map(|(i, byte)| if i & 1 != 0 { double_num(byte) } else { byte })
        .sum::<i32>()
        % 10
        == 0
}

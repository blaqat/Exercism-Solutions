#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    } else if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    } else if let Some(&invalid) = number.iter().find(|&&num| num >= from_base) {
        return Err(Error::InvalidDigit(invalid));
    }

    let mut num = number.iter().fold(0_u32, |acc, n| acc * from_base + n);

    if num == 0 {
        return Ok(vec![0]);
    }

    Ok(std::iter::from_fn(|| {
        if num > 0 {
            let rem = num.rem_euclid(to_base);
            num /= to_base;
            Some(rem)
        } else {
            None
        }
    })
    .collect::<Vec<u32>>()
    .into_iter()
    .rev()
    .collect())
}

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn eval(expr: &str) -> i64 {
    expr.split('+')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .sum()
}

fn solve_alphametic(input: &str) -> Option<HashMap<char, char>> {
    let chars = input
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect::<HashSet<char>>();

    let parts = input
        .split_terminator("==")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let (left_eq, right_eq) = (parts[0], parts[1]);

    let left_parts: Vec<&str> = left_eq.split_terminator('+').map(|s| s.trim()).collect();
    let leading_chars = left_parts
        .iter()
        .chain(Some(&right_eq))
        .map(|w| w.chars().next().unwrap())
        .collect::<Vec<char>>();

    for perm in "0123456789".chars().permutations(chars.len()) {
        if leading_chars
            .iter()
            .any(|&ch| perm[chars.iter().position(|&c| c == ch).unwrap()] == '0')
        {
            continue;
        }

        let trans: HashMap<char, char> = chars.iter().cloned().zip(perm.iter().cloned()).collect();
        let new_left_eq: String = left_eq
            .chars()
            .map(|c| {
                trans
                    .iter()
                    .find(|(&ch, _)| ch == c)
                    .map(|(_, d)| *d)
                    .unwrap_or(c)
            })
            .collect();
        let new_right_eq: String = right_eq
            .chars()
            .map(|c| {
                trans
                    .iter()
                    .find(|(&ch, _)| ch == c)
                    .map(|(_, d)| *d)
                    .unwrap_or(c)
            })
            .collect();

        // dbg!(&trans, &new_left_eq, &new_right_eq);

        if eval(&new_left_eq) == new_right_eq.parse::<i64>().unwrap() {
            return Some(trans);
        }
    }

    None
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // dbg!(input, left_eq, right_eq, left_parts, chars, leading_chars);
    Some(
        solve_alphametic(input)?
            .into_iter()
            .map(|(x, c)| (x, c.to_digit(10).unwrap() as u8))
            .collect::<HashMap<char, u8>>(),
    )

    // unimplemented!("Solve the alphametic {input:?}")
    // None
}

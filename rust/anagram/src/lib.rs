use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower: String = word.to_lowercase();
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort_unstable();

    possible_anagrams
        .iter()
        .filter_map(|&w| {
            let w_lower: String = w.to_lowercase();

            let mut w_sorted: Vec<char> = w_lower.chars().collect();
            w_sorted.sort_unstable();

            if word_lower != w_lower && w_sorted == word_sorted {
                Some(w)
            } else {
                None
            }
        })
        .collect()
}

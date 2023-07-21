pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c| matches!(c, ' ' | '-' | '_'))
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|ch| ch.is_ascii_uppercase())
                    .filter(|ch| ch.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}

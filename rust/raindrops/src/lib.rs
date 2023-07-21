const SOUNDS: [(u32, &str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    SOUNDS
        .into_iter()
        .filter_map(|(i, sound)| match n % i {
            0 => Some(sound.to_string()),
            _ => None,
        })
        .reduce(|acc, s| acc + &s)
        .unwrap_or(n.to_string())
}

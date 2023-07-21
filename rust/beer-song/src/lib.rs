//Capitalizes first letter of a string
fn to_title_case(s: &str) -> String {
    format!("{}{}", &s[0..1].to_uppercase(), &s[1..])
}
pub fn verse(n: u32) -> String {
    let get_bottles_of_beer = |amt: u32| -> String {
        let mut s = "s";
        let mut n = amt.to_string();
        match amt {
            0 => n = "no more".to_string(),
            1 => s = "",
            _ => {}
        };
        format!("{n} bottle{s} of beer")
    };
    let what_to_do = match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };
    let bottles_of_beer = get_bottles_of_beer(n);
    let new_bottles_of_beer = get_bottles_of_beer(n.checked_sub(1).unwrap_or(99));

    format!(
        "{Bottles_of_beer} on the wall, {bottles_of_beer}.\n{what_to_do}, {new_bottles_of_beer} on the wall.\n",
        Bottles_of_beer = to_title_case(&bottles_of_beer)
    )
}
pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}

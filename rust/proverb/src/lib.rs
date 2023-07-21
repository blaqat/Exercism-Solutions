pub fn build_proverb(list: &[&str]) -> String {
    if let Some(first) = list.first() {
        list.windows(2)
            .map(|items| format!("For want of a {} the {} was lost.\n", items[0], items[1]))
            .chain(Some(format!("And all for the want of a {first}.")))
            .collect()
    } else {
        String::new()
    }
}

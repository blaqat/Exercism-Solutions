pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with('?');
    let has_a_letter = message.find(char::is_alphabetic).is_some();
    let is_yelling = has_a_letter && message.chars().all(|chr| !chr.is_lowercase());

    if is_questioning {
        return if is_yelling {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        };
    }
    if is_yelling {
        return "Whoa, chill out!";
    }

    "Whatever."
}

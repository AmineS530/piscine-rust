pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    format!(
        "{}{}",
        (&input[..1].to_string()).to_uppercase(),
        &input[1..]
    )
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| capitalize_first(word))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}

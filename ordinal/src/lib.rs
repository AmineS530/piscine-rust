pub fn num_to_ordinal(x: u32) -> String {
    let mod10 = x % 10;
    let mod100 = x % 100;

    let suffix = match (mod10, mod100) {
        (1, n) if n != 11 => "st",
        (2, n) if n != 12 => "nd",
        (3, n) if n != 13 => "rd",
        _ => "th",
    };

    format!("{x}{suffix}")
}

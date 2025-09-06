fn grep_num(s: &str) -> i32 {
    for c in s.chars() {
        if c.is_numeric() {
            return c as i32 - 0x30;
        }
    }
    0
}
fn remove_digits(s: &str) -> String {
    s.chars().filter(|c: &char| !c.is_ascii_digit()).collect()
}

pub fn arrange_phrase(phrase: &str) -> String {
    let mut splitted_phrase: Vec<&str> = phrase.split_whitespace().collect();
    splitted_phrase.sort_by_key(|s| grep_num(s));
    remove_digits(&splitted_phrase.join(" "))
}

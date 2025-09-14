use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new();

    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            letters.insert(c);
        }
    }

    letters.len() == 26
}

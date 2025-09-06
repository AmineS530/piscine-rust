pub fn first_subword(mut s: String) -> String {
    if let Some(idx) = s.find('_') {
        return s[0..idx].to_owned();
    } else {
        for (i, c) in s.chars().enumerate().skip(1) {
            if c.is_uppercase() {
                return s.drain(..i).collect();
            }
        }
    }
    s
}

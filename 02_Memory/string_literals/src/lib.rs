pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    if let Some(idx) = v.find(pat) {
        return idx;
    }
    return 0;
}

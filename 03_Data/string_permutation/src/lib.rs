pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut c1: Vec<_> = s1.chars().collect();
    let mut c2: Vec<_> = s2.chars().collect();
    c1.sort_unstable();
    c2.sort_unstable();
    c1 == c2
}
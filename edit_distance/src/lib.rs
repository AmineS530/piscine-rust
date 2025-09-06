pub fn edit_distance(source: &str, target: &str) -> usize {
    source
        .chars()
        .zip(target.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

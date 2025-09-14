pub fn score(s: &str) -> String {
    let mut count = 0;
    for c in s.chars() {
        match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => {
                count += 1;
            }
            'D' | 'G' => {
                count += 2;
            }
            'B' | 'C' | 'M' | 'P' => {
                count += 3;
            }
            'F' | 'H' | 'V' | 'W' | 'Y' => {
                count += 4;
            }
            'K' => {
                count += 5;
            }
            'J' | 'X' => count += 8,
            'Q' | 'Z' => count += 10,
            _ => count += 0,
        }
    }
    count.to_string()
}

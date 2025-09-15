pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    text.split_whitespace()
        .map(|word| {
            let chars: Vec<char> = word.chars().collect();

            // Starts with a vowel
            if vowels.contains(&chars[0]) {
                return format!("{word}ay");
            }

            // Look for consonant cluster or "qu"
            let mut idx = 0;
            while idx < chars.len() {
                // If we find "qu", we treat both as one unit
                if idx != 0 && idx + 1 < chars.len() && chars[idx] == 'q' && chars[idx + 1] == 'u' {
                    idx += 2;
                    break;
                }

                // If we hit a vowel, break
                if vowels.contains(&chars[idx]) {
                    break;
                }

                idx += 1;
            }

            // Rearranging
            let (start, end) = chars.split_at(idx);
            let new_word: String = end.iter().chain(start.iter()).collect();
            format!("{new_word}ay")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

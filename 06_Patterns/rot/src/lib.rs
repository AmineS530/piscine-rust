pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        result.push(caesar_cipher(c, key));
    }
    result
}

fn caesar_cipher(c: char, key: i8) -> char {
    let key = ((key % 26 + 26) % 26) as u8;

    match c {
        'A'..='Z' => ((c as u8 - b'A' + key as u8) % 26 + b'A') as char,
        'a'..='z' => ((c as u8 - b'a' + key as u8) % 26 + b'a') as char,
        _ => c,
    }
}

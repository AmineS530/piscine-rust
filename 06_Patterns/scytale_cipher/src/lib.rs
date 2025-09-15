pub fn scytale_cipher(message: &str, i: usize) -> String {
    let len = message.len();
    if len == 0 || i == 0 {
        return String::new();
    }

    let cols = (len + i - 1) / i;

    let mut chars: Vec<char> = message.chars().collect();

    // Pad with spaces to fill the grid
    while chars.len() < cols * i {
        chars.push(' ');
    }

    let mut res = String::new();
    for row in 0..i {
        for col in 0..cols {
            let idx = col * i + row;
            res.push(chars[idx]);
        }
    }

    res.trim().to_string()
}

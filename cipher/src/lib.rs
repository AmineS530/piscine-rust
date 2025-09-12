#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

fn atbash(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
            'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
            _ => c,
        })
        .collect()
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = atbash(original);
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}

pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut str_vec = Vec::with_capacity(names.len());

    for name in names.iter() {
        let mut res = String::with_capacity(5);
        for (i, word) in name.split_whitespace().take(2).enumerate() {
            if let Some(c) = word.chars().next() {
                res.push(c);
                res.push('.');
                if i == 0 {
                    res.push(' ');
                }
            }
        }
        str_vec.push(res);
    }
    str_vec
}

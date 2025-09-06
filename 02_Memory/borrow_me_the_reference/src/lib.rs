pub fn delete_and_backspace(s: &mut String) {
    let mut delete_counter = 0;

    let res: String = s.chars().fold(String::new(), |mut acc, c| {
        match c {
            '+' => {
                delete_counter += 1;
            }
            '-' => {
                acc.pop();
            }
            _ => {
                if delete_counter > 0 {
                    delete_counter -= 1;
                } else {
                    acc.push(c);
                }
            }
        }
        acc
    });

    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    for op in v.iter_mut() {
        let parts: Vec<&str> = if op.contains('+') {
            op.split('+').collect()
        } else {
            op.split('-').collect()
        };
        let first = parts[0].parse::<i32>().unwrap_or(0);
        let second = parts[1].parse::<i32>().unwrap_or(0);

        let result = if op.contains('+') {
            first + second
        } else {
            first - second
        };
        *op = result.to_string();
    }
}

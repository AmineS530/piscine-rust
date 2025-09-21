pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut my_box = Vec::new();
    s.split_whitespace().for_each(|slice| {
        let current = parse_number(slice);
        my_box.push(Box::new(current));
    });
    my_box
}

fn parse_number(s: &str) -> u32 {
    if let Some(trimmed) = s.strip_suffix('k') {
        (trimmed.parse::<f64>().unwrap() * 1000.0).round() as u32
    } else {
        s.parse::<u32>().unwrap()
    }
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|elem| *elem).collect::<Vec<u32>>()
}

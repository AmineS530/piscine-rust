use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 0 {
        (sorted[len / 2] + sorted[len / 2 - 1]) / 2
    } else {
        sorted[len / 2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Safe unwrap because list is assumed non-empty
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0
}

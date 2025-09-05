pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c_f = c as f64;
    (c, c_f.exp(), c_f.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let res = a
        .chars()
        .filter_map(|c| c.to_string().parse::<f64>().ok())
        .map(|val| val.exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (a, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let res = b.iter().map(|val| (val.abs() as f64).ln()).collect();
    (b, res)
}

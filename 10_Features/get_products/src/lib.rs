pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return vec![];
    }
    let n = arr.len();
    let mut result = vec![1; n];
    let mut prod = 1;
    for i in 0..n {
        result[i] = prod;
        prod *= arr[i];
    }

    prod = 1;
    for i in (0..n).rev() {
        result[i] *= prod;
        prod *= arr[i];
    }

    result
}
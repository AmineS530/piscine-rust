pub fn number_logic(num: u32) -> bool {
    let n_len = num_len(num) as u32;
    let mut sum = 0;
    let mut temp = num;

    while temp > 0 {
        let digit = temp % 10;
        sum += digit.pow(n_len);
        temp /= 10;
    }

    sum == num
}

fn num_len(mut num: u32) -> u8 {
    let mut len: u8 = 1;
    while num > 10 {
        len += 1;
        num /= 10;
    }
    len
}

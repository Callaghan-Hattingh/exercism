pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vec_digits = Vec::new();
    if digits.len() < len {
        return vec_digits;
    }
    for ser in 0..=(digits.len() - len) {
        // println!("series: {}", ser);
        vec_digits.push((&digits[ser..(ser + len)]).parse().unwrap());
        // println!("digits: {}", &digits[ser..(ser + len)]);
    }
    vec_digits
}

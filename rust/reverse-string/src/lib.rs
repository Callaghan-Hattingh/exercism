pub fn reverse(input: &str) -> String {
    let mut s3 = String::new();
    for i in input.chars() {
        s3.insert(0, i)
    }
    return s3;
}
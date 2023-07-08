use std::ptr::write;

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime = Vec::new();
    let mut cur_num = n;

    while cur_num != 1 {
        for num in 2..=n {
            let remainder = cur_num % num;
            if remainder == 0 {
                prime.push(num);
                cur_num /= num;
                break;
            }
        }
    }
    prime
}

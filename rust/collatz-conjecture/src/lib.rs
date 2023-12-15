pub fn collatz(n: u64) -> Option<u64> {
    let mut num = n;
    if n < 1 {
        None
    } else {
        let mut steps: u64 = 0;
        while num > 1 {
            steps+=1;
            if num % 2 == 1 {
                num = num.checked_mul(3)?.checked_add(1)?;
            } else {
                num /= 2;
            }
        }
        Some(steps)
    }
}

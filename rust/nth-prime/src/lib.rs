pub fn nth(n: u32) -> u32 {
    let n = n + 1;
    let mut list_of_primes: Vec<u32> = vec![2];
    while list_of_primes.len() < n as usize {
        let mut test_num = list_of_primes[list_of_primes.len() - 1] + 1;
        while is_prime(test_num) == false {
            test_num += 1;
        }
        list_of_primes.push(test_num);
    }
    return list_of_primes[list_of_primes.len() - 1];
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut divisor = 2;
    while divisor * divisor <= n {
        if n % divisor == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}

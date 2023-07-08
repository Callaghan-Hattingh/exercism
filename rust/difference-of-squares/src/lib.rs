use num::{pow, range};

pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    pow(sum, 2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += pow(i, 2)
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    let squ_sum = square_of_sum(n);
    let sum_squ = sum_of_squares(n);
    squ_sum - sum_squ
}

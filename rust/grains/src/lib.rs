pub fn square(s: u32) -> u64 {
    return (chess_double(s)).into();
}

pub fn total() -> u64 {
    let mut total = 0;
    let count = 65;
    for i in 1..count {
            let t1 = chess_double(i);
            total+=t1;
    }
    return total;
}

fn chess_double(n: u32) -> u64 {
    if n < 1 {
        panic!("Square must be between 1 and 64")
    } else if n > 64 {
        panic!("Square must be between 1 and 64")
    }

    if n == 1 {
        1
    } else {
        2 * chess_double(n - 1)
    }
}
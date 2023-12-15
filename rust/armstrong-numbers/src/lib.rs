use num_traits::pow;

pub fn is_armstrong_number(num: u32) -> bool {
    let string_num = num.to_string();
    let index = string_num.len();
    let num: u64 = num as u64;
    let mut total: u64 = 0;
    println!("{index}, {num}");

    for c in string_num.chars() {
        println!("{total}");
        total += pow(c.to_string().parse::<u64>().unwrap(), index);
    }
    println!("{}", total);
    if total == num {
        true
    } else {
        false
    }
}

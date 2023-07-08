pub fn is_leap_year(year: u64) -> bool {
    let mut ans = false;
    if year % 400 == 0 {ans = true}
    else if year % 100 == 0 { ans = false}
    else if year % 4 == 0 {ans = true }
    ans
}

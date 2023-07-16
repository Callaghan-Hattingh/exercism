pub fn raindrops(n: u32) -> String {
    let mut rv = String::new();

    if n % 3 == 0 {
        rv.push_str("Pling")
    }
    if n % 5 == 0 {
        rv.push_str("Plang")
    }
    if n % 7 == 0 {
        rv.push_str("Plong")
    }
    if rv.len() == 0 {
        rv.push_str(&format!("{}", n));
    }
    rv
}

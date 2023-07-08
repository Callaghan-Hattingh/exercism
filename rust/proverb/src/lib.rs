pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut proverb = String::new();
    let mut one = list[0];
    for input in &list[1..] {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", one, input));
        one = input;
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}

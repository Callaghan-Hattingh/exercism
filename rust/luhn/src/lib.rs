/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped_code = code.replace(" ", "");

    if stripped_code.len() < 2 {
        false
    }
    else {
        let non_luhn: String = code.to_owned().chars().rev().filter(|c| !c.is_numeric()).collect();
        if !non_luhn.is_empty() {
            return false;
        }
        let luhn: String = code.to_owned().chars().rev().filter(|c| c.is_numeric()).collect();
        println!("{}", luhn);

        let s1: Vec<(usize, char)> = luhn.chars().enumerate().filter_map(|(index, c)| {
            if index % 2 == 1 {
                println!("{:?}", (index, c));
                let mut num_c = c as u8 - 48;
                num_c*=2;
                if num_c > 9 {
                    num_c-=9;
                }
                num_c+=48;
                println!("{:?}", num_c);
                Some((index, num_c as char))
                // Some((index, String::from(format!("{}", num_c)).chars().last().unwrap()))
            } else {
                Some((index, c))
            }
        }).collect();
        println!("{:?}", s1);

        let mut total = 0;
        for (_, c) in s1.iter() {
            let u_c = c.clone() as u8 -48;
            total+=u_c;
        }
        println!("{:?}", total);
        total % 10 == 0
    }
}

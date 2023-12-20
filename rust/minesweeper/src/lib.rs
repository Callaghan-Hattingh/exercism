use std::vec;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        // return vec![String::new()];
        return Vec::new();
    }
    if minefield.len() == 1 && minefield == [""] {
            return vec![String::new()];
        }
    let x_max = minefield.len() as i32;
    let y_max = minefield[0].len() as i32;

    let mut ans = Vec::new();
    let mut ans_row = String::new();
    let mut vals = Vec::new();
    let mut mines = Vec::new();
    for (outer_ind, row) in minefield.iter().enumerate() {
        for (inner_ind, c) in row.chars().into_iter().enumerate() {
            if c == ' ' {
                ans_row.push(' ');
                // vals.push((outer_ind, inner_ind));
            }
            else if c == '*' {
                ans_row.push('*');
                mines.push((outer_ind as i32, inner_ind as i32));           
            }

        }
        ans.push(ans_row.to_owned());
        ans_row.clear();
        println!("{:?}", row);
        
    }
    // println!("{:?}", mines);
    let check_mines = mines.clone();


    for mine in mines {
        for i in [-1, 0, 1] {
            for q in [-1, 0 , 1] {
                let x = mine.0 - i;
                let y = mine.1 - q;

                match x {
                    -1 => continue,
                    w if w >= x_max => continue,
                    _ => {}
                }

                match y {
                    -1 => continue,
                    e if e >= y_max => continue,
                    _ => {}
                }

                if check_mines.contains(&(x, y)) {
                    continue;
                }

                vals.push((x, y));
            }
        }
    }
    vals.sort();
    // println!("{:?}", (x_max, y_max));

    // println!("{:?}", vals);

    for num in vals {
        // println!("{:?}", ans[num.0 as usize]);
        let row =  &ans[num.0 as usize];
        let value = row.chars().nth(num.1 as usize); 
        if value.unwrap() == ' ' {
            unsafe {
                let fin = ans[num.0 as usize].as_mut_vec();
                fin[num.1 as usize] = b'1';
            }
        }
        else {
            let calc = value.unwrap() as i32 + 1;
            unsafe {
                let fin = ans[num.0 as usize].as_mut_vec();
                fin[num.1 as usize] = calc as u8;
            }
        }
        
    }

    ans
}

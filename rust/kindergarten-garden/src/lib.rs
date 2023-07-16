use std::ops::Index;
use std::str::Chars;

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let mut student_plants_short: Vec<char> = Vec::new();
    let mut student_plants: Vec<&str> = Vec::new();
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let exist = students.contains(&_student);

    if exist {
        let index = students.iter().position(|&x| x == _student).unwrap();
        let col_nums = [index * 2, index * 2 + 1];
        // println!("cols: {:?}", col_nums);
        // println!("diagram: {:?}", _diagram.clone().chars());

        let row2: String = _diagram
            .chars()
            .skip_while(|&c| c != '\n')
            .skip(1)
            .collect();

        // println!("row2: {:?}", row2.chars());

        for col_num in col_nums {
            let value = _diagram.chars().nth(col_num).unwrap();
            // println!("{}", value);
            student_plants_short.push(value);
        }
        for col_num in col_nums {
            let value = row2.chars().nth(col_num).unwrap();
            // println!("{}", value);
            student_plants_short.push(value);
        }

        // println!("short: {:?}", student_plants_short);

        for c in student_plants_short {
            if c == 'V' {
                student_plants.push("violets");
            } else if c == 'R' {
                student_plants.push("radishes");
            } else if c == 'C' {
                student_plants.push("clover")
            } else {
                student_plants.push("grass")
            }
        }
    }

    // println!("long: {:?}", student_plants);

    student_plants
}

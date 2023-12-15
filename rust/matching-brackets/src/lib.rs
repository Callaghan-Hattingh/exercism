// pub fn brackets_are_balanced(string: &str) -> bool {
//     let mut q: i32 = 0;
//     let mut w: i32 = 0;
//     let mut e: i32 = 0;
//     println!("{string}");
//     for i in string.chars() {
//         println!("{i}");
//         match i {
//             '{' => q += 1,
//             '}' => q -= 1,
//             '[' => w += 1,
//             ']' => w -= 1,
//             '(' => e += 1,
//             ')' => e -= 1,
//             _ => continue,
//         }
//
//         if q == -1 || w == -1 || e == -1 {
//             break;
//         }
//     }
//     let mut c_string = string.clone();
//
//     if q == 0 && w == 0 && e == 0 {
//         true
//     } else {
//         false
//     }
// }


pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    println!("{string}");
    for c in string.chars() {
        println!("{c}");
        println!("{:?}", stack);
        match c {
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            ']' | '}' | ')' if stack.pop() != Some(c) => return false,
            _ => (),
        }
    }
    stack.is_empty()
}
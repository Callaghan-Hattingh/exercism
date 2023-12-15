// use std::ops::Add;

pub fn verse(n: u32) -> String {
    if n == 0 {
        return String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\
        \nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        );
    } else if n == 1 {
        return String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\
            \nTake it down and pass it around, no more bottles of beer on the wall.\n",
        );
    } else if n == 2 {
        let m = n - 1;
        return String::from(format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\
        \nTake one down and pass it around, {m} bottle of beer on the wall.\n"
        ));
    }
    let m = n - 1;
    return String::from(format!(
        "{n} bottles of beer on the wall, {n} bottles of beer.\
    \nTake one down and pass it around, {m} bottles of beer on the wall.\n"
    ));
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for i in (end..=start).rev() {
        song.push_str(&verse(i));
        song.push_str("\n");
    }
    song.truncate(song.len() - 1);
    return song;
}

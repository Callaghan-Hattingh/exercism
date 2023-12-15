// use core::unicode::conversions::to_lower;
use std::collections::HashSet;


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    for ana in possible_anagrams {
        if word.len() == ana.len() {
            let cp_ana = ana.to_lowercase();
            let cp_word = word.to_lowercase();

            let mut bytes_ana = Vec::new();
            for c in cp_ana.chars() {
                // println!("{:?}", c);
                let mut buf = [0; 4]; // Create a buffer to store the bytes
                let bytes = c.encode_utf8(&mut buf);
                // println!("{:?}", bytes.as_bytes());
                bytes_ana.push(bytes.as_bytes().to_owned());
            }
            bytes_ana.sort();
            // println!("sort {:?}",bytes_ana);

            let mut bytes_word = Vec::new();
            for c in cp_word.chars() {
                // println!("{:?}", c);
                let mut buf = [0; 4]; // Create a buffer to store the bytes
                let bytes = c.encode_utf8(&mut buf);
                // println!("{:?}", bytes.as_bytes());
                bytes_word.push(bytes.as_bytes().to_owned());
            }
            bytes_word.sort();
            // println!("sort {:?}",bytes_word);

            
            // let uni_ana = cp_ana.as_bytes();
            // let uni_word = cp_word.as_bytes();
            // println!("{:?}",uni_ana);
            // let mut sort_ana = uni_ana.to_owned();
            // let mut sort_word = uni_word.to_owned();
            // sort_ana.sort();
            // sort_word.sort();

            // let ana_chars = ana.chars();
            // let mut lower_ana = Vec::new();
            // for c in ana_chars {
            //     lower_ana.push(c.to_ascii_lowercase());
            // }
            // let mut char_sort_ana = lower_ana.clone();
            // char_sort_ana.sort();

            // let word_chars = word.chars();
            // let mut lower_word = Vec::new();
            // for c in word_chars {
            //     lower_word.push(c.to_ascii_lowercase());
            // }
            // let mut char_sort_word = lower_word.clone();
            // char_sort_word.sort();


            if cp_ana == cp_word {
                break;
            }
            else if bytes_ana == bytes_word {
                anagrams.insert(ana.to_owned());
            }
            // if lower_ana == lower_word {
            //     break;
            // } 
            // else if sort_ana == sort_word || char_sort_ana == char_sort_word {
            //     anagrams.insert(ana.to_owned());
            // }
            // else if char_sort_ana == char_sort_word {
            //     anagrams.insert(ana.to_owned());
            // }
        }
    }
    anagrams
}
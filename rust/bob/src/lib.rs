pub fn reply(message: &str) -> &str {
    let copy_message = message.clone();
    let clean = remove_special_characters_iter(copy_message);
    let captial = is_uppercase_string(clean.as_str());

    if message.trim().len() == 0 {
        return "Fine. Be that way!";
    } else if captial {
        if message.trim().chars().last().unwrap() == '?' {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    } else if message.trim().chars().last().unwrap() == '?' {
        return "Sure.";
    }
    return "Whatever.";
}

fn remove_special_characters_iter(s: &str) -> String {
    s.chars().filter(|c| c.is_alphabetic()).collect()
}

fn is_uppercase_string(s: &str) -> bool {
    if s.len() == 0 {
        return false;
    }
    s.chars().all(|c| c.is_uppercase())
}

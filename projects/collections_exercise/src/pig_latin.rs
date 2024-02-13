pub fn to_pig_latin(s: &String) -> String {
    let mut pig_latin_s = String::new();

    for word in s.split_whitespace() {
        if word.is_empty() {
            continue;
        }

        let first_char = word.chars().next().unwrap();

        if ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&first_char) {
            pig_latin_s.push_str(&format!("{word}-hay "));
        } else {
            let mut word_chars = word.chars();
            word_chars.next();
            let exclude_first_char: String = word_chars.collect();
            pig_latin_s.push_str(&format!("{exclude_first_char}-{first_char}ay "));
        }
    }

    pig_latin_s.trim_end().to_string()
}

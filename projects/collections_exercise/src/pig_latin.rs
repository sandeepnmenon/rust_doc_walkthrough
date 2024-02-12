pub fn to_pig_latin(s: &String) -> String {
    let mut pig_latin_s = String::new();

    for word in s.split_whitespace() {
        if word.starts_with(&['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']) {
            pig_latin_s.push_str(&format!("{word}-hay "));
        } else {
            let first_char = word.chars().next().unwrap_or(' ');
            let exclude_first_char = &word[1..];
            pig_latin_s.push_str(&format!("{exclude_first_char}-{first_char}ay "));
        }
    }

    pig_latin_s.trim_end().to_string()
}

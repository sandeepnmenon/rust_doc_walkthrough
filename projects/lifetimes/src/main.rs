use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Excerpt is '{}'", self.part)
    }
}

fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let longest_str = longest(string1.as_str(), string2);
    println!("The longest string is {longest_str}");

    let string1 = String::from("abcd");
    let longest_str;
    {
        let string2 = String::from("x"); // Note: If this was a string literal there is no issue as it has a static lifetime.
        longest_str = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {longest_str}"); // If this line is moved out of this scope it does not compile as the lifetime of string2 is not valid outside.
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt: ImportantExcerpt<'_> = ImportantExcerpt {
        part: first_sentence,
    };

    let longest_str = longest_with_an_announcement(novel.as_str(), first_sentence, excerpt);
    println!("The longest string is '{longest_str}'");
}

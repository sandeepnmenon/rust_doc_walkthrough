fn main() {
    println!("The Slice type!");
    // A slice is a kind of reference, so it does not have ownership.

    let mut s = String::from("hello world");
    let word = first_word_basic(&s);
    println!("First word end index is: {}", word);
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally logically invalid!
               // We have to worry about the index in word being relevant in the program as the string s changes.
               // The solution to this in Rust is slices!

    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // Some syntaxes
    let s = String::from("hello");
    let len = s.len();

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let _slice = &s[3..len];
    let _slice = &s[3..];

    let s = String::from("hello world");
    let frst_word = first_word(&s);
    println!("First word is: {}", frst_word);

    let scnd_word = second_word(&s);
    println!("Second word is: {}", scnd_word);

    // Changed `fn first_word(s: &String) -> &str {`
    // to `fn first_word(s: &str) -> &str {`
    // it allows us to use the same function on both &String values and &str values.

    let my_string = String::from("hello world");
    let my_string_literal = "Hello, world!";

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("First word is: {}", word);

    let word = first_word(&my_string[..]);
    println!("First word is: {}", word);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word(&my_string);

    // `first_word` works on slices of string literals, whether partial or whole
    let _word = first_word(&my_string_literal[0..6]);

    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    // Slices can be of other datatypes too

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word_basic(s: &String) -> usize {
    let bytes = s.as_bytes(); // To go through the string element by element.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let first_word = first_word(&s);
    let after_first_word_index = first_word.len();

    if after_first_word_index >= s.len() {
        return "";
    }

    let bytes = s[after_first_word_index + 1..].as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[after_first_word_index + 1..after_first_word_index + 1 + i];
        }
    }

    &s[after_first_word_index + 1..]
}

fn main() {
    println!("The Slice type!");
    // A slice is a kind of reference, so it does not have ownership.

    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally logically invalid!
    // We have to worry about the index in word being relevant in the program as the string s changes.
    // The solution to this in Rust is slices!


    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();   // To go through the string element by element.

    for (i, &item) in bytes.iter().enumerate() {
        if item == ' ' {
            return i;
        }
    }

    s.len()
}
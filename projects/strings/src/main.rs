fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    // Can also be written as
    let s = "initial contents".to_string();
    //Also
    let s = String::from("initial contents");

    // Strings are UTF-8 encoded so any language is possible
    let hello = String::from("नमस्ते");

    println!("{}", s);
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    //Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);
    // + operator uses fn add(self, s: &str) -> String {
    // We can only add a &str to a String, Cannot add two String values
    // Compiler coerces &String into &str. Called deref coercion which turns &s2 into &s2[..]

    // For more complicated string combining.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // Or
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    //Indexing
    // Rust strings dont support indexing
    //Eg let h = s[0] // Doesnt work
    // A String is a wrapper over a Vec<u8>

    // Rather than indexing, we can create slices
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}");
    // output is Зд because each of the character is 2 bytes and requesting [0..4] requests first 4 bytes of the string
    // Below line would panic because we are trying to slice a part of the character's bytes
    // let s = &hello[0..1];

    // To get individual Unicode scalar values, iterate
    for c in hello.chars() {
        print!("{c} ");
    }
    println!();
    // For bytes
    for c in hello.bytes() {
        print!("{c} ");
    }
    println!();
    //Getting grapheme clusters from strings as with the Devanagari script is complex,
    // so this functionality is not provided by the standard library. Crates available though
}

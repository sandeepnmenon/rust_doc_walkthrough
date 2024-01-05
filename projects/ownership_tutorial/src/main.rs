fn main() {
    {
        // _s is not valid here as its not yet declared.
        // Using _ prefix to prevent unused variable warning. Just for demonstration.
        let _s = "hello world"; // _s is valid from here and till remaining of the block
    } // the scope is over and _s is no longer valid

    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // appends a literal to a String
        println!("{}", s);
    } // Memory of s in the heap is freed using the special `drop` function. This pattern is similar to RAII.

    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);

    let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1); // Does not work because the s1 was moved to s2.
    // But if we do a clone, a duplicate memory space is created in the heap and the data copied over.
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // If you see a clone, some arbitrary code is being executed and that code may be expensive

    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid here because it was moved to the function

    let x = 5;
    makes_copy(x);
    // x is valid here because it was copied. i32 has the Copy trait.

    let s1 = gives_ownership(); // s1 gets ownership
    println!("s1 = {}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which then moves it back to s3 by returning
    println!("s3 = {}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    // But this is too much hassle if we just want to pass a value to a function, do some computation to it and return another value and still getting to use the passed value further on
    // So RUST has references. For using a value without transferring ownership

    let s1 = String::from("hello with ref");
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn takes_ownership(some_string: String) {
    println!("{} ", some_string);
} // some_string goes out of scope and `drop` is called to free the memory it was pointing to.

fn makes_copy(some_int: i32) {
    println!("{} ", some_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours initialized in gives_ownership");
    some_string // moves the memory to the variable that gives_ownership is assigned to
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

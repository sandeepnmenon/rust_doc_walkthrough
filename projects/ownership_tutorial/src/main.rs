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
    let len = calculate_length_ref(&s1);    // This is borrowing. The variable in the function does not own it.
    println!("The length of '{}' is {}.", s1, len);

    let s = String::from("hello");
    // change_ref(&s);  // This function attempts to modify the value of the modified reference. References are immutable by default.
    
    // To allow for mutable reference. Given example.
    let mut s = String::from("hello");
    change_mut_ref(&mut s);

    // We can only have one mutable reference to a value.
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("Mutable reference r1 of s: {}", r1);
    // The above mechanism is important to avoid data races.
    // We can create multiple mutable references but not simultaneously.

    // Also cannot have a mutable reference while we have an immutable reference to the same value.
    let r2 = &s;
    let r3 = &mut s;    
    // println!("{}, {}, and {}", r1, r2, r3); // No can do, if we try to access them in the same scope.

    // However, the below example works. Because the references scope starts from where it was created to the last point where it is referenced.
    // The compiler can undertand that.
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Rust does not allow Dangling references
    // let reference_to_nothing = dangle();


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

// This does not compile as it attempts to change a reference, which is immutable.
// fn change_ref(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change_mut_ref(some_string: &mut String) {
    some_string.push_str(", world");

}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // Returning a reference
// }   // s goes out of scope and dropped. Now the reference does not point to anything. Hence Rust does not allow this.
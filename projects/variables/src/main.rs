fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; // Only possible because x is mutable
    println!("The value of x is {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);

    // Integer overflow
    let overflow: u8 = 255;
    println!("The value of overflow is: {}", overflow);
    // let overflow = overflow + 1 ;
    // println!("The value of overflow is: {}", overflow);
    // let overflow = overflow + 1;
    // println!("The value of overflow is: {}", overflow);

    // Compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    println!("The value of tup is: {:#?}", tup);

    let (_x, y, _z) = tup;
    println!("The value of tup.1 is: {}", y);
    println!("The value of tup.0 is: {}", tup.0);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);
    let a = [3; 5];
    println!("The value of a is: {:?}", a);
    println!("The value of a[0] is: {}", a[0]);
    println!("The value of a[1] is: {}", a[1]);
    // Rust panics if you try to access an element outside of the array
}

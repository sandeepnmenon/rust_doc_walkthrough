fn main() {
    let v: Vec<i32> = Vec::new();

    //convenient macro
    let v = vec![1, 2, 3];

    // Updating a vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading from a vector
    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // let does_not_exist = &v[100]; // This causes Rust to panic
    let _does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    v.push(6);

    // Does not run because v is mutable and an immutable reference of v is borrowed
    // println!("The first element is: {first}");

    //Iterating
    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    // Using enum to store multipl types
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text((String::from("Pi"))),
    ];
    // This is so that RUST needs to know what types will be in the vector at compile time.
    // If we dont know, then enum wont work and we need to use the trait object which is not explained in this project

    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}

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
}

fn main() {
    println!("Hello, world!");

    let y =  {
        let x = 6;
        x + 1
    };

    println!("The value of y is: {y}");
    another_function(9, 'h');

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
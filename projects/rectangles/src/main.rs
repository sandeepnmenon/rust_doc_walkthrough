#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Version 1
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    // Version 2 with tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect1)
    );

    // Version 3 with structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rect1)
    );

    println!("Rect1 is {:?}", rect1);
    println!("Rect1 is {:#?}", rect1); // Prettier printing

    // Another way is to use the !dbg macro
    // this takes ownership of the value and then returns it as opposed to println!
    // Also this prints to stderr as opposed to stdout
    // Also prints line number and file path
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    // Using the area method of Rectangle
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

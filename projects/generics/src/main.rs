fn find_largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for value in &list[1..] {
        if value > largest {
            largest = value;
        }
    }

    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for value in &list[1..] {
        if value > largest {
            largest = value;
        }
    }

    largest
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for value in &list[1..] {
        if value > largest {
            largest = value;
        }
    }

    largest
}

fn main() {
    let arr = vec![1, 2, 4, 8, 16, 32, 64, 128];
    let largest = find_largest_i32(&arr);
    println!("The largest number is {}", largest);

    let arr = vec![1, 2, 4, 6, 8, 10, 12, 14];
    let largest = find_largest_i32(&arr);
    println!("The largest number is {}", largest);

    let arr = vec!['a', 'c', 'z', 'b'];
    let largest = find_largest_char(&arr);
    println!("The largest number is {}", largest);

    // With generic function
    let arr = vec![1, 2, 4, 8, 16, 32, 64, 128];
    let largest = find_largest(&arr);
    println!("The largest number is {}", largest);

    let arr = vec!['a', 'c', 'z', 'b'];
    let largest = find_largest(&arr);
    println!("The largest number is {}", largest);

    // Structs with generics
    let _integer_point = Point { x: 5, y: 9 };
    let _float_point = Point { x: 5.0, y: 9.0 };
    let _integer_float_point = Point { x: 5, y: 9.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p = Point {
        x: 5.0_f32,
        y: 10.0_f32,
    };
    println!("Distance from origin: {}", p.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Rust generics are fast
    // Rust uses monomorphization: compiles all generics during compile time and generates code for each type of generic.
    let integer = Some(5);
    let float = Some(5.0);
    //Gets compiled to
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.x.powi(2)).sqrt()
    }
}

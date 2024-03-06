fn find_largest(list: &[i32]) -> &i32 {
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
    let largest = find_largest(&arr);
    println!("The largest number is {}", largest);

    let arr = vec![1, 2, 4, 6, 8, 10, 12, 14];
    let largest = find_largest(&arr);
    println!("The largest number is {}", largest);
}

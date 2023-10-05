fn main() {
    println!("Enter n to get the nth fibonacci number:");

    let n = read_natural_number();
    let nth_fibo = get_nth_fibo(n);
    let ordinal = get_ordinal(n);
    println!("The {}{} fibonacci number is {}", n, ordinal, nth_fibo);
}

fn read_natural_number() -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<i32>() {
            Ok(num) if num > 0 => break num,
            Ok(_) => println!("Please enter a positive number"),
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn get_ordinal(n: i32) -> String {
    match n % 10 {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
    .to_string()
}

fn get_nth_fibo(n: i32) -> i32 {
    return if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        get_nth_fibo(n - 1) + get_nth_fibo(n - 2)
    };
}

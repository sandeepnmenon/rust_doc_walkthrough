use std::io;

fn main() {
    println!("Enter temperature in Fahrenheit:");

    let fahrenheit: f32 = read_number();
    let celsius = fahrenheit_to_celsius(fahrenheit);

    println!("{}°F is {}°C", fahrenheit, celsius);
}

fn read_number() -> f32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        }
    }
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
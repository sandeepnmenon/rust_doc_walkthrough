pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    #[test]
    fn exploration() {
        assert_eq!(add(3, 3), 6);
    }

    #[test]
    #[ignore = "This test panics"]
    fn failing_test() {
        panic!("Make this test fail");
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(3), 5);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

mod rectangle_tests {
    use super::Rectangle;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod greeting_tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        match value {
            ..1 | 101.. => panic!("Guess value must be between 1 and 100, got {value}"),
            1..=100 => Guess { value },
        }
    }
}

#[cfg(test)]
mod guess_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod show_output_tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore = "this test will fail"]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}

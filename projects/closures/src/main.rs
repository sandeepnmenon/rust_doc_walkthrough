use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_reds = 0;
        let mut num_blues = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_reds += 1,
                ShirtColor::Blue => num_blues += 1,
            }
        }

        if num_reds > num_blues {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference = Some(ShirtColor::Red);
    let giveaway = store.giveaway(user_preference);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference, giveaway
    );

    let user_preference = None;
    let giveaway = store.giveaway(user_preference);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference, giveaway
    );

    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    // let n = example_closure(5);// Will not compile as the compile can only assign one type for closure params.

    let list = vec![1, 2, 3];
    println!("Before defining closure : {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = vec![3, 2, 1];
    println!("Before defining closure : {list:?}");
    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {list:?}"); // Does not compile as list is borrowed immutably here but it is also borrowed mutable by the closure.
    borrows_mutably();
    println!("After calling closure: {list:?}");

    println!("Before calling closure for spawn thread: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    // println!("After calling closure for spawn thread: {list:?}");//Wont compile as list is borrowed/moved by the thread.


    let mut list = [
       Rectangle { width:10 , height:1 }, 
       Rectangle { width:3 , height:5 },
       Rectangle { width:7 , height:12 }, 
    ];
    list.sort_by_key(|x|x.width);
    println!("{list:#?}");

    let mut num_sort_operations = 0;
    list.sort_by_key(|x| {
        num_sort_operations+=1;
        x.height
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    let some_change = Coin::Quarter(UsState::Alaska);
    println!("I have change of {} cents", value_in_cents(&some_change));

    let some_change = Coin::Penny;
    println!("I have change of {} cents", value_in_cents(&some_change));

    // Using match to  handle Option<T>
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // Catch-all patterns and _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // `_ => ()`, // for no op
    }

    // if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to me {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // We can use if let and else to do the same
    // let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

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
}

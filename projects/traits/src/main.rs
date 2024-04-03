mod aggregator;
use aggregator::{Journal, Summary, Tweet};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // Default train
    let journal = Journal {
        title: String::from("Test journal"),
        author: String::from("Me"),
        content: String::from("No content"),
    };

    println!("New journal availabel!: {}", journal.summarize());

    notify(&tweet);
}

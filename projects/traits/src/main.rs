mod aggregator;
use aggregator::{Journal, Summary, Tweet};

mod pair;
use pair::Pair;

// pub fn notify<T:Summary>(item: &T){  // Equivalent verbose way of the below impl Trait syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("penguin_ebooks"),
        content: String::from("Penguin folks"),
        reply: true,
        retweets: 0,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweets: 0,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // Testing the Display trait on Tweet
    println!("{}", tweet);

    // Testing the Summary trait on Vec<T>
    let test_vector = vec![1..3];
    println!("{}", test_vector.summarize());
    println!("{}", test_vector.summarize_author());

    // Default trait
    let journal = Journal {
        title: String::from("Test journal"),
        author: String::from("Me"),
        content: String::from("No content"),
    };

    println!("New journal availabel!: {}", journal.summarize());

    notify(&tweet);

    // Returning of type Trait
    let tweet = returns_summarizable();
    notify(&tweet);

    // Trait bounds to conditionally implement methods
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweets: 0,
    };
    let tweet1 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know, people"),
        reply: false,
        retweets: 1,
    };
    let tweet_pair: Pair<Tweet> = Pair::new(tweet, tweet1);
    println!("Tweet pair: {tweet_pair}");
    tweet_pair.cmp_display();

}

use std::{cmp::Ordering, fmt};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweets: u32,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-------------------")?;
        writeln!(f, "@{}", self.username)?;
        writeln!(f, "{}", self.content)?;
        writeln!(f, "Reply: {}", self.reply)?;
        writeln!(f, "Retweets: {}", self.retweets)?;
        write!(f, "-------------------")
    }
}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.retweets == other.retweets
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.retweets.cmp(&other.retweets) {
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
        }
    }
}

pub struct Journal {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Journal {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl<T> Summary for Vec<T> {
    fn summarize(&self) -> String {
        format!("Vector has {} elements.", self.len())
    }

    fn summarize_author(&self) -> String {
        format!("You are the author bro!")
    }
}

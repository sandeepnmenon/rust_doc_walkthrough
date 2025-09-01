use std::fmt;

pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    pub fn new(f: T, s: T) -> Self {
        Self {
            first: f,
            second: s,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "First\n: {}\n Second\n: {}", self.first, self.second)
    }
}

impl<T: fmt::Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.first >= self.second {
            println!(
                "First value {} is greater than Second value {}",
                self.first, self.second
            );
        } else {
            println!(
                "Second value {} is greater than First value {}",
                self.second, self.first
            );
        }
    }
}

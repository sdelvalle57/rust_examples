use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//This is a way
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

//with trait bounds
fn notify_t_b<T: Summary>(item: &T) {
    println!("Breaking news from trait bounds! {}", item.summarize())
}

//with trait bounds and where clauses
fn notify_t_b_w<T>(item: &T) where T: Summary {
    println!("Breaking news from trait bounds and where! {}", item.summarize())
}

fn largest<T>(list: &[T]) -> T
where T: PartialOrd + Copy {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}



fn main() {
    let tweet = Tweet {
        username: String::from("sdelvalle"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    notify_t_b(&tweet);
    notify_t_b_w(&tweet);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);



}

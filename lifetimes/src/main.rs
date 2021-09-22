use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> Display for ImportantExcerpt<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Display {}", self.part);
        write!(f, "({})", self.part)
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("level: {}", i.level());
    println!("announcement: {}", i.announce_and_return_part("warning"));

    let largest_announcement = longest_with_an_announcement("hola", "sisas", i);
    println!("largest_announcement: {}", largest_announcement);
}

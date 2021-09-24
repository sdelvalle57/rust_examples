use core::fmt;
use std::fmt::{Display, write};



trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String
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

/**************************************************/
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

/**************************************************/
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/**************************************************/


trait OutlinePrint {
    fn outline_print(&self);
}

struct Point {
    x: i32,
    y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

 /**************************************************/

 struct Wrapper(Vec<String>);

 impl fmt::Display for Wrapper {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "[{}]", self.0.join(", "))
     }
 }

 /**************************************************/

 trait Draw {
     fn draw(&self);
 }

 /** A struct that receives a vec of generic types which implements Draw */

 struct ScreenD<T: Draw> {
     components: Vec<T>
 }

 impl<T> ScreenD<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
 } 

 
 struct Screen {
     components: Vec<Box<dyn Draw>>
 }

 impl Screen {
     pub fn run(&self) {
         for component in self.components.iter() {
             component.draw();
         }
     }
 }



 

fn main() {
    let tweet = Tweet {
        username: String::from("sdelvalle"),
        content: String::from(
            "of course, as you probably already know, people",
        )
    };

    notify(&tweet);
    notify_t_b(&tweet);
    notify_t_b_w(&tweet);

    /**************************************************/

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    /**************************************************/


    let person = Human;
    Wizard::fly(&person);
    person.fly();

    Pilot::fly(&person);
    //or
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);
    <Human as Pilot>::fly(&person);

     /**************************************************/

    let point = Point {
        x: 10,
        y: 10
    };
    point.outline_print();


     /**************************************************/

     let w = Wrapper(vec![String::from("hello"), String::from("world"), String::from("beautiful"), String::from("world")]);
     println!("w = {}", w);
}

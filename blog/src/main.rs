use blog::post_1::Post as Post1;
use blog::post_2::Post as Post2;

fn main() {
    let mut post1 = Post1::new();

    post1.add_text("I ate salad for lunch today");
    assert_eq!("", post1.content());

    post1.request_review();
    assert_eq!("", post1.content());

    post1.approve();
    assert_eq!("I ate salad for lunch today", post1.content());

    let mut post2 = Post2::new();
    post2.add_text("I ate salad for lunch today");

    let post2 = post2.request_review();
    let post2 = post2.approve();
    assert_eq!("I ate salad for lunch today", post2.content());


}

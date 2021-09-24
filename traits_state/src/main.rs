use std::fmt::Debug; 



trait State: Debug { 
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Post {
    state: Option<Box<dyn State>>,
    content: String
}



impl Post {
    pub fn new() -> Post {
        Post { 
            state: Some(Box::new(Draft {})), 
            content: String::new() 
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}



fn main() {

    //Inits as Draft
    let mut post = Post::new();

    println!("Post state after creating {:?}", post.state); //Draft

    post.add_text("I ate arepa for breakfast today");
    assert_eq!("", post.content());

    //Changes to PendingReview
    post.request_review();
    println!("Post state after request review {:?}", post.state); //pendingReview

    //approves review
    post.approve();
    println!("Post state after approve {:?}", post.state); //Published

    println!("Post content {}", post.content);
}

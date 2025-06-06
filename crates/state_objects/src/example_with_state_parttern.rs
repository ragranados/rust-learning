use std::fmt::Debug;
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    num_aproves: i8,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            num_aproves: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {

        if !self.state.as_ref().unwrap().is_allow_to_change_content() {
            return;
        }

        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn print_state(&self) {
        println!("Estado: {:?}", &self.state)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn aprove(&mut self) {
        self.num_aproves += 1;

        if self.num_aproves < 2 {
            return;
        };

        if let Some(s) = self.state.take() {
            self.state = Some(s.aprove())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State: Debug {

    fn is_allow_to_change_content(&self) -> bool;

    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn aprove(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {

    fn is_allow_to_change_content(&self) -> bool  {
        true
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn aprove(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {

    fn is_allow_to_change_content(&self) -> bool {
        false
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn aprove(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {

    fn is_allow_to_change_content(&self) -> bool {
        false
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn aprove(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

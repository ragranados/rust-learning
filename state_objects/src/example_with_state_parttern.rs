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
        if let Some(s) = self.state.take() {
            self.state = Some(s.aprove(self))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State: Debug {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn aprove(self: Box<Self>, post: &mut Post) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn aprove(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn aprove(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        post.num_aproves += 1;

        if post.num_aproves == 2 {
            Box::new(Published {})
        } else {
            self
        }
    }
}

#[derive(Debug)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn aprove(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

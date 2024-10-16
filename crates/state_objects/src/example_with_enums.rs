enum State {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    state: State,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: State::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Draft => "",
            State::PendingReview => "",
            State::Published => &self.content,
        }
    }

    pub fn request_review(&mut self) {
        match self.state {
            State::Draft => self.state = State::PendingReview,
            State::PendingReview => {}
            State::Published => {}
        }
    }

    pub fn aprove(&mut self) {
        match self.state {
            State::Draft => {}
            State::PendingReview => self.state = State::Published,
            State::Published => {}
        }
    }
}

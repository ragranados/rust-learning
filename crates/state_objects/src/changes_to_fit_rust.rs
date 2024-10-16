pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> FirstPendingReviewPost {
        FirstPendingReviewPost {
            content: self.content,
        }
    }
}

pub struct FirstPendingReviewPost {
    content: String,
}

impl FirstPendingReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn request_review(self) -> SecondPendingReviewPost {
        SecondPendingReviewPost {
            content: self.content,
        }
    }
}

pub struct SecondPendingReviewPost {
    content: String,
}

impl SecondPendingReviewPost {
    pub fn aprove(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

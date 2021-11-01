use super::Post;

pub trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn get_content<'a>(&self, post: &'a Post) -> &'a str {
    "" // Return empty string by default
  }
}

pub struct Draft {}
impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {}) // Update state to PendingReview
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

pub struct PendingReview {}
impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {}) // Update state to Published
  }
}

pub struct Published {}
impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn get_content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

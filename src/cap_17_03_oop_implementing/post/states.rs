use super::Post;

pub trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn get_content<'a>(&self, post: &'a Post) -> &'a str {
    "" // Return empty string by default
  }
  fn reject(self: Box<Self>) -> Box<dyn State>;
}

pub struct Draft {}
impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview { approvals: 0 }) // Update state to PendingReview
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

pub struct PendingReview {
  pub approvals: i8, // Other states agnostic about approvals
}
impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
  fn approve(self: Box<Self>) -> Box<dyn State> {
    let new_approvals = self.approvals + 1;

    if new_approvals > 1 {
      Box::new(Published {})
    } else {
      Box::new(PendingReview {
        approvals: new_approvals,
      })
    }
  }
  fn reject(self: Box<Self>) -> Box<dyn State> {
    Box::new(Draft {})
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
  fn reject(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

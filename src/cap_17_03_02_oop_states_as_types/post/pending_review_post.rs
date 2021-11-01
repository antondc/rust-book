use super::Post;

pub struct PendingReviewPost {
  pub content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

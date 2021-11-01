use super::PendingReviewPost;

pub struct DraftPost {
  pub content: String,
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

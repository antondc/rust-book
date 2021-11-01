use super::{Draft, State};

// A post can be in one of three states: Draft, PendingReview or Published
// These states are defined on post_states module
// Post will be initialized as a Draft {}
pub struct Post {
  state: Option<Box<dyn State>>,
  pub content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  // Its possible to add text in any of the states
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  // The behaviour of this function is tied to its state
  pub fn get_content(&self) -> &str {
    self.state.as_ref().unwrap().get_content(self)
  }

  // The behaviour of this function is tied to its state
  pub fn request_review(&mut self) {
    if let Some(state) = self.state.take() {
      self.state = Some(state.request_review())
    }
  }

  // The behaviour of this function is tied to its state
  pub fn approve(&mut self) {
    if let Some(state) = self.state.take() {
      self.state = Some(state.approve())
    }
  }

  pub fn reject(&mut self) {
    if let Some(state) = self.state.take() {
      self.state = Some(state.reject())
    }
  }
}

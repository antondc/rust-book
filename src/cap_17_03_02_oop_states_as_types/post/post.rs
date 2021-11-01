use super::DraftPost;

// Post subtypes are now typed and implemented as independent types
// Each type method will return another type when it is transformed
pub struct Post {
  pub content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn get_content(&self) -> &str {
    &self.content
  }
}

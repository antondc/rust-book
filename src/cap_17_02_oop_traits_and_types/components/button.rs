use super::super::types::Draw;

pub struct Button {
  pub height: i32,
  pub width: i32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!(
      "Drawing a button with height {}, width {} and label {}",
      self.height, self.width, self.label
    );
  }
}
